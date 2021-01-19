#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use std::fmt;

pub struct Green<T>(T);

impl<T> fmt::Display for Green<T> where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1b[0;32m{}\x1b[0m", self.0)
    }
}

fn main() {
    println!("This is Alan {}span", Green("Green"));
}
