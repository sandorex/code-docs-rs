#[allow(dead_code)]
pub trait DocumentedFields {
    fn field_types() -> Vec<&'static str>;
    fn field_names() -> Vec<&'static str>;
    fn field_docs() -> Vec<Vec<&'static str>>;
}

#[macro_export]
macro_rules! code_docs {
    (
        $(#[$($m:meta)?])*
        $p:vis struct $name:ident {
            $(
                $(#[$($im:meta)?])*
                $p2:vis $i:ident : $t:ty ,
            )*
        }
    ) => {
            $(#[$($m)?])*
            $p struct $name {
                $(
                    $(#[$($im)?])*
                    $p2 $i : $t ,
                )*
            }

        impl DocumentedFields for $name {
            fn field_types() -> Vec<&'static str> {
                vec![
                    $(
                        stringify!($t),
                    )*
                ]
            }

            fn field_names() -> Vec<&'static str> {
                vec![
                    $(
                        stringify!($i),
                    )*
                ]
            }

            fn field_docs() -> Vec<Vec<&'static str>> {
                vec![
                    $(
                        vec![
                            $(
                                $(stringify!($im),)?
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

    code_docs! {
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
