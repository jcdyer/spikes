pub struct Intervals<I> {
    iter: I,
    prev: u64,
}

impl<I> Intervals<I> {
    pub fn new(mut iter: I) -> Option<Intervals<I>>
    where
        I: Iterator<Item = u64>,
    {
        let initial = iter.next()?;
        Some(Intervals::with_initial(iter, initial))
    }

    pub fn with_initial(iter: I, initial: u64) -> Intervals<I> {
        Intervals {
            iter,
            prev: initial,
        }
    }
}

impl<I> Iterator for Intervals<I>
where
    I: Iterator<Item = u64>,
{
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let (prev, next) = (self.prev, self.iter.next()?);
        self.prev = next;
        Some(next - prev)
    }
}

#[test]
fn test_intervals() {
    let mut int = Intervals::new(super::Sieve::new().into_iter()).unwrap();
    assert_eq!(int.next(), Some(1), "3");
    assert_eq!(int.next(), Some(2), "5");
    assert_eq!(int.next(), Some(2), "7");
    assert_eq!(int.next(), Some(4), "11");
    assert_eq!(int.next(), Some(2), "13");
    assert_eq!(int.next(), Some(4), "17");
    assert_eq!(int.next(), Some(2), "19");
    assert_eq!(int.next(), Some(4), "23");
    assert_eq!(int.next(), Some(6), "29");
    assert_eq!(int.next(), Some(2), "31");
    assert_eq!(int.next(), Some(6), "37");

}
