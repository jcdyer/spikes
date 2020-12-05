// Understand the Infallible type

#![feature(crate_visibility_modifier, exhaustive_patterns)]



mod never {
    use std::convert::Infallible;
    crate fn always() -> Result<i32, Infallible> {
        Ok(42)
    }
}

#[derive(Clone, Debug)]
crate struct Vec2D<T> {
    inner: Vec<Vec<T>>,
    dim: (usize, usize),
}

#[macro_use]
macro_rules! vec2d {
    ( $( $x:expr ),* ) => {{
        let mut inner = Vec::new();
        let mut a;
        $({
            a = $x;
            inner.push(a.to_vec());
        })*
        let cols = inner.len();
        let rows = inner.iter().next().map(|row| row.len()).unwrap_or(0);

        Vec2D {
            inner,
            dim: (cols, rows),
        }
    }}
}



fn main() {

    let Ok(x) = never::always();
    println!("{}", x);

    #[allow(clippy::infallible_destructuring_match)]
    let x = match never::always() {
        Ok(x) => x,
    };
    println!("{}", x);

    let square = vec2d![
        [1,2,3],
        [2,3,4],
        [3,4,5],
        [4,5,6]
    ];

    println!("{:#?}", square);


}
