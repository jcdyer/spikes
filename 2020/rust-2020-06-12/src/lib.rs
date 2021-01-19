//! bignum::U is an unsized, unsigned integer type.
//!
//! Goal: Implement the following traits:
//!     * From<uN>,
//!     * From<&str> for hex & decimal,
//!     * Add
//!     * Sub
//!     * Mul
//!     * Div (long division algorithm?  Others?)
//!
//! Stretch goal: Implement bignum::I

pub mod bignum {

    use std::convert::TryFrom;

    #[derive(Debug)]
    pub enum Error {
        ConvertError,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct U(Vec<u8>);

    impl U {
        /// normalize strips trailing 0 bytes from the bignum::U.  All (safe)
        /// public functions must leave the bignum::U in normalized form.
        fn normalize(&mut self) {
            let mut ct = self.0.iter().rev().take_while(|item| **item == 0).count();
            while ct > 0 {
                self.0.pop();
                ct -= 1;
            }
        }

        pub fn try_from_hex(mut number: &str) -> Result<U, Error> {
            let mut v = Vec::with_capacity(number.len() / 2 + 1); // TODO: Check boundary conditions
            while !number.is_empty() {
                let offset = if number.len() < 2 {
                    0
                } else {
                    number.len() - 2
                };
                let byte = &number[offset..];
                number = &number[..offset];
                v.push(u8::from_str_radix(byte, 16).map_err(|_| Error::ConvertError)?);
            }
            let mut u = U(v);
            u.normalize();
            Ok(u)
        }

        pub fn try_from_dec(_number: &str) -> Result<U, Error> {
            // Cannot implement without Add
            todo!()
        }

        pub fn try_from_bin(_number: &str) -> Result<U, Error> {
            todo!()
        }

        pub fn with_capacity(n: usize) -> Self {
            Self(Vec::with_capacity(n))
        }

        pub fn try_to_u64(&self) -> Option<u64> {
            if self.0.len() > 8 {
                None
            } else {
                Some(
                    self.0
                        .iter()
                        .enumerate()
                        .map(|(place, igit)| 256u64.pow(place as u32) * (*igit as u64))
                        .sum(),
                )
            }
        }
    }

    impl Default for U {
        fn default() -> Self {
            Self(Vec::new())
        }
    }

    impl TryFrom<&str> for U {
        type Error = Error;
        fn try_from(number: &str) -> Result<Self, Error> {
            match &number[..2] {
                "0x" => Self::try_from_hex(&number[2..]),
                "0b" => Self::try_from_bin(&number[2..]),
                _ => Self::try_from_dec(number),
            }
        }
    }

    impl std::ops::Add<U> for U {
        type Output = U;
        fn add(self, other: U) -> U {
            self.add(&other)
        }
    }

    impl std::ops::Add<&U> for U {
        type Output = U;
        fn add(mut self, other: &U) -> U {
            let (otherbase, otherextra) = other.0.split_at(self.0.len());

            let mut carry = false;
            // Create an infinite iterator from other
            let other_iter = otherbase.iter().copied().chain(std::iter::repeat(0));

            // Add to each igit of self the corresponding igit of other, carrying ones as needed
            for (this, addend) in self.0.iter_mut().zip(other_iter) {
                let (value, overflow) = this.overflowing_add(addend);
                let (value, overflow) = if carry {
                    value.overflowing_add(1)
                } else {
                    (value, overflow)
                };
                *this = value;
                carry = overflow;
            }

            // If other was longer than self, push the remaining igits of other, carrying ones as needed.
            for addend in otherextra {
                let (value, overflow) = if carry {
                    addend.overflowing_add(1)
                } else {
                    (*addend, false)
                };
                self.0.push(value);
                carry = overflow;
            }

            // If there was a overflow at the last igit of other, add it
            if carry {
                self.0.push(1);
            }
            self
        }
    }

    impl std::ops::Add<U> for &U {
        type Output = U;

        fn add(self, other: U) -> U {
            self + &other
        }
    }

    impl std::ops::Add<&U> for &U {
        type Output = U;

        // Is this allocation badly implicit?
        fn add(self, other: &U) -> U {
            let new = U::with_capacity(self.0.len().max(other.0.len()) + 1);
            new + self + other
        }
    }

    #[cfg(test)]
    mod tests {
        use super::U;
        use std::convert::TryFrom;

        #[test]
        fn it_works() -> Result<(), super::Error> {
            let u1 = U::try_from("0xf456")?;
            let u2 = U::try_from("0x1234")?;
            let sum = U::try_from("0x1068a")?;
            assert_eq!(u1 + u2, sum);
            Ok(())
        }

        #[test]
        fn normalization() -> Result<(), super::Error> {
            // bypassing public constructor to build a denormalized bignum::U
            let mut u1 = U(vec![1, 0, 0, 0, 0, 0, 0]);
            let u2 = U(vec![1]);
            assert_eq!(u1, u2);
            u1.normalize();
            assert_eq!(u1, u2);
            Ok(())
        }
    }
}
