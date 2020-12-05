use std::{ops::Index};


/// Sieve of Eratosthenes.
pub struct Sieve {
    primes: Vec<u64>,
}

impl Sieve {
    pub fn new() -> Sieve {
        Sieve::with_cache(Vec::new())
    }

    pub fn with_capacity(cap: usize) -> Sieve {
        Sieve::with_cache(Vec::with_capacity(cap))
    }

    pub fn with_cache(mut primes: Vec<u64>) -> Sieve {
        if primes.is_empty() {
            primes.push(2);
            primes.push(3);
        } else {
            assert_eq!(&primes[..2], &[2, 3]);
        }
        Sieve { primes }
    }

    pub fn nth(&mut self, idx: usize) -> u64 {
        let mut candidate = *self.primes.last()
            .expect("constructor ensures vec has len >= 1");


        while idx >= self.primes.len() {
            'nextprime: loop {
                candidate += 1;
                for &prime in &self.primes {
                    if candidate % prime == 0 {
                        continue 'nextprime;
                    }
                }
                self.primes.push(candidate);
                break;

            }
        }
        *self.primes.index(idx)
    }
}

impl Default for Sieve {
    fn default() -> Sieve {
        Sieve::new()
    }
}

impl IntoIterator for Sieve {
    type IntoIter = SieveIntoIter;
    type Item = u64;

    fn into_iter(self) -> Self::IntoIter {
        SieveIntoIter { sieve: self, index: 0 }
    }
}
pub struct SieveIntoIter {
    sieve: Sieve,
    index: usize,
}

impl Iterator for SieveIntoIter {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let index = self.index;
        self.index += 1;

        Some(self.sieve.nth(index))
    }
}

pub mod intervals;
pub mod prime_factors;
#[test]
fn test_primes() {
    let mut sieve = Sieve::new();
    assert_eq!(sieve.nth(0), 2);
    assert_eq!(sieve.nth(1), 3);
    assert_eq!(sieve.nth(2), 5);
    assert_eq!(sieve.nth(3), 7);
    assert_eq!(sieve.nth(4), 11);
    assert_eq!(sieve.nth(5), 13);
    assert_eq!(sieve.nth(6), 17);
    assert_eq!(sieve.nth(99), 541);
}
