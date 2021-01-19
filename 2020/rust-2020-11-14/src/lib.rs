
pub trait DoubleIterExt<I, T> where Self: Sized {
    fn doubled(self) -> DoubleIter<I, T>;
}

impl<I> DoubleIterExt<I, I::Item> for I
where
    I: Iterator,
{
    fn doubled(self) -> DoubleIter<I, I::Item> {
        DoubleIter {
            iter: self,
            next: None,
        }
    }
}

pub struct DoubleIter<I, T> {
    iter: I,
    next: Option<T>,
}

impl<I> Iterator for DoubleIter<I, I::Item>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> {
        self.next.take()
            .or_else(|| {
                self.next = self.iter.next();
                self.next.clone()
            })
    }
}

#[cfg(test)]
mod tests {
    use super::DoubleIter;
    use super::DoubleIterExt;
    #[test]
    fn double_iter() {
        let mut doubler = DoubleIter {
            iter: ::std::iter::once(4u64),
            next: None::<u64>,
        };
        assert_eq!(doubler.next(), Some(4));
        assert_eq!(doubler.next(), Some(4));
        assert_eq!(doubler.next(), None);
        assert_eq!(doubler.next(), None);
        assert_eq!(doubler.next(), None);
    }

    #[test]
    fn double_iter_clone_type() {
        let mut doubler = DoubleIter{
            iter: ::std::iter::once(String::from("utf-8")),
            next: None::<String>,
        };
        assert_eq!(doubler.next(), Some(String::from("utf-8")));
        assert_eq!(doubler.next(), Some(String::from("utf-8")));
        assert_eq!(doubler.next(), None);
        assert_eq!(doubler.next(), None);
        assert_eq!(doubler.next(), None);
    }

    #[test]
    fn double_iter_ext() {
        let mut doubler = vec!["this", "or", "that"].into_iter().doubled();
        assert_eq!(doubler.next(), Some("this"));
        assert_eq!(doubler.next(), Some("this"));
        assert_eq!(doubler.next(), Some("or"));
        assert_eq!(doubler.next(), Some("or"));
        assert_eq!(doubler.next(), Some("that"));
        assert_eq!(doubler.next(), Some("that"));
        assert_eq!(doubler.next(), None);
    }
}
