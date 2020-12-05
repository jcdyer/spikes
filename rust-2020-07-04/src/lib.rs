
#[macro_export]
macro_rules! boxslice {
    ( $( $x:expr ),* ) => {{
        vec![
            $($x),*
        ].into_boxed_slice()
    }}
}

#[cfg(test)]
mod tests {
    use super::boxslice;

    #[test]
    fn it_works() {
        let boxed: Box<[u16]> = boxslice![1,4,3,2,7,6,9,1032,4];
        assert_eq!(boxed[4], 7);
        println!("{:?}", boxed);
        panic!()
    }
}
