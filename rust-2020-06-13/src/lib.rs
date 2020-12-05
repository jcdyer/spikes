    pub mod svec {
        #![macro_use]

        #[macro_export]
        macro_rules! svec {
            () => {{
                let v = Vec::<String>::new();
                v

            }};
            ($($elem:expr),+ $(,)?) => {{
                let v = vec![
                    $( String::from($elem), )*
                ];
                v
            }};
        }

        #[cfg(test)]
        mod tests {
            #[test]
            fn it_works() {
                let v = dbg!(svec!["this", "that", "the other"]);
                let x: Vec<String> = v;
                assert_eq!(x, vec![
                           String::from("this"),
                           String::from("that"),
                           String::from("the other"),
                ]);
                assert!(false);
            }
        }
    }
