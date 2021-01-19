trait ErrIntoExt<T0, E0> {
    fn err_into<E>(self) -> Result<T0, E>
    where
        E: From<E0>;
}

impl<T0, E0> ErrIntoExt<T0, E0> for Result<T0, E0> {
    fn err_into<E>(self) -> Result<T0, E>
    where
        E: From<E0>,
    {
        self.map_err(E::from)
    }
}


type ErrMapFn<T0, E0, E> = fn(Result<T0, E0>,) -> Result<T0, E>;

trait ResultIterExt<T0, E0>: Iterator<Item = Result<T0, E0>> + Sized {
    fn errs_into<E>(self) -> std::iter::Map<Self, ErrMapFn<T0, E0, E>>
    where
        E: From<E0>;
}

impl<I0, T0, E0> ResultIterExt<T0, E0> for I0
where
    I0: Iterator<Item = Result<T0, E0>>,
    Self: Sized,
{
    fn errs_into<E>(self) -> std::iter::Map<Self, ErrMapFn<T0, E0, E>>
    where
        E: From<E0>,
    {
        self.map(|res| res.map_err(E::from))
    }
}

#[cfg(test)]
mod tests {
    use super::{ResultIterExt, ErrIntoExt};
    use std::str::FromStr;

    #[derive(Debug, PartialEq, Eq)]
    struct TestError;
    impl From<std::num::ParseIntError> for TestError {
        fn from(_: std::num::ParseIntError) -> TestError {
            TestError
        }
    }

    #[test]
    fn err_into() {
        assert_eq!("32".parse::<i32>().err_into::<TestError>(), Ok(32));
        assert_eq!("abc".parse::<i32>().err_into::<TestError>(), Err(TestError));
    }

    #[test]
    fn errs_into() {
        assert_eq!(
            "12|fourteen|16".split('|')
                .map(u32::from_str)
                .errs_into::<TestError>()
                .collect::<Result<Vec<_>, _>>(),
            Err(TestError),
        )
    }
}
