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
                 .filter(|y| y.starts_with("doc = r\""))
                 .map(|y| &y[8..y.len()-1])
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
                        .filter(|y| y.starts_with("doc = r\""))
                        .map(|y| &y[8..y.len()-1])
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

    code_docs_struct! {
        #[derive(PartialEq, Eq, Debug)]
        /// This is a testing struct
        /// With two lines of docstring
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
}
