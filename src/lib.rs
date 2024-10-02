#[allow(dead_code)]
pub trait DocumentedStruct {
    /// Returns all docstrings of the struct itself
    fn struct_docs() -> Vec<&'static str>;

    /// Returns all struct field types, ex. `Option<String>`
    fn field_types() -> Vec<&'static str>;

    /// Returns all struct field names
    fn field_names() -> Vec<&'static str>;

    /// Returns all docstrings of each field, each line is a separate string
    fn field_docs() -> Vec<Vec<&'static str>>;

    // TODO add nesting so if the type is documented as well then include it
    /// Returns formatted string showing types of each field and comments above it, the format is
    /// very similar to raw rust
    fn commented_fields() -> Result<String, Box<dyn std::error::Error>> {
        use std::fmt::Write;

        let mut output = String::new();
        let names = Self::field_names();
        let docs = Self::field_docs();
        let types = Self::field_types();

        // just in case so the errors are not so cryptic below
        assert_eq!(names.len(), docs.len(), "Field names and field docs are not equal");
        assert_eq!(names.len(), types.len(), "Field names and field types are not equal");

        for (i, field) in names.iter().enumerate() {
            // remove the extra newline
            if i != 0 {
                write!(output, "\n")?;
            }

            // imitate rust comments
            for comment in docs.get(i).unwrap() {
                write!(output, "///{}\n", comment)?;
            }

            write!(output, "{}: {}\n", field, types.get(i).unwrap())?;
        }

        Ok(output)
    }
}

pub fn filter_docs(meta: &str) -> Option<&str> {
    // i do not want to use regex crate
    meta.strip_prefix("doc = r\"")
        .or(meta.strip_prefix("doc =\nr\"")) // there was a newline before comment
        .and_then(|x| x.strip_suffix("\"")) // remove the quotes
}

#[macro_export]
macro_rules! code_docs_struct {
    (
        $(#[$($meta:meta)?])*
        $vis:vis struct $name:ident {
            $(
                $(#[$($f_meta:meta)?])*
                $f_vis:vis $f_ident:ident : $f_type:ty ,
            )*
        }
    ) => {
            $(#[$($meta)?])*
            $vis struct $name {
                $(
                    $(#[$($f_meta)?])*
                    $f_vis $f_ident : $f_type ,
                )*
            }

        impl DocumentedStruct for $name {
            fn struct_docs() -> Vec<&'static str> {
                vec![
                    $(
                        $(
                            stringify!($meta),
                        )?
                    )*
                ].into_iter()
                 .filter_map($crate::filter_docs)
                 .collect::<Vec<_>>()
            }

            fn field_types() -> Vec<&'static str> {
                vec![
                    $(
                        stringify!($f_type),
                    )*
                ]
            }

            fn field_names() -> Vec<&'static str> {
                vec![
                    $(
                        stringify!($f_ident),
                    )*
                ]
            }

            fn field_docs() -> Vec<Vec<&'static str>> {
                vec![
                    $(
                        vec![
                            $(
                                $(stringify!($f_meta),)?
                            )*
                        ],
                    )*
                ].into_iter().map(|x| {
                    x.into_iter()
                     .filter_map($crate::filter_docs)
                     .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // NOTE: the newline below is intentional
    code_docs_struct! {

        /// This is a testing struct
        /// With two lines of docstring
        #[derive(PartialEq, Eq, Debug)]
        struct TestStruct {
            /// This field is u8
            pub field_a: u8,

            /// This field is a String
            /// Also two lines of comments
            #[allow(unused)]
            field_b: String,
        }
    }

    #[test]
    fn test_struct_docs() {
        let docs = TestStruct::struct_docs();
        let docs_raw: Vec<&'static str> = vec![
            " This is a testing struct",
            " With two lines of docstring",
        ];

        assert_eq!(docs, docs_raw);
    }

    #[test]
    fn test_field_names() {
        let names = TestStruct::field_names();
        let names_raw: Vec<&'static str> = vec![
            "field_a",
            "field_b",
        ];

        assert_eq!(names, names_raw);
    }

    #[test]
    fn test_field_types() {
        let types = TestStruct::field_types();
        let types_raw: Vec<&'static str> = vec![
            "u8",
            "String",
        ];

        assert_eq!(types, types_raw);
    }

    #[test]
    fn test_field_docs() {
        let docs = TestStruct::field_docs();
        let docs_raw: Vec<Vec<&'static str>> = vec![
            vec![" This field is u8"],
            vec![
                " This field is a String",
                " Also two lines of comments",
            ],
        ];

        assert_eq!(docs, docs_raw);
    }

    #[test]
    fn test_commented_fields() {
        let output = TestStruct::commented_fields();
        assert!(output.is_ok());

        assert_eq!(
            output.unwrap(),
            r#"/// This field is u8
field_a: u8

/// This field is a String
/// Also two lines of comments
field_b: String
"#
        );
    }
}
