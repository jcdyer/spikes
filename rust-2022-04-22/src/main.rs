use std::fmt::Debug;

pub trait Apply: Sized {
    fn apply<U>(self, f: impl FnOnce(Self) -> U) -> U;
}

impl<T> Apply for T {
    fn apply<U>(self, f: impl FnOnce(T) -> U) -> U {
        f(self)
    }
}

fn get_err<T: Debug, E>(res: Result<T, E>) -> E {
    res.unwrap_err()
}

fn main() {
    let x = std::str::from_utf8(&[1, 2, 138]).apply(get_err);
    println!("ERROR: {:?}", x);
}
