mod into_iter {
    use std::iter::{IntoIterator, Iterator};

    struct Bleh {
        first: u32,
        second: u32,
        third: u32,
    }

    #[derive(Copy, Clone)]
    enum IterState {
        First,
        Second,
        Third,
        Done,
    }

    struct BlehIntoIter {
        bleh: Bleh,
        state: IterState,
    }

    impl IntoIterator for Bleh {
        type Item = u32;
        type IntoIter = BlehIntoIter;

        fn into_iter(self) -> BlehIntoIter {
            BlehIntoIter {
                bleh: self,
                state: IterState::First,
            }
        }
    }

    impl Iterator for BlehIntoIter {
        type Item = u32;

        fn next(&mut self) -> Option<u32> {
            Some(match self.state {
                IterState::First => {
                    self.state = IterState::Second;
                    self.bleh.first
                }
                IterState::Second => {
                    self.state = IterState::Third;
                    self.bleh.second
                }
                IterState::Third => {
                    self.state = IterState::Done;
                    self.bleh.third
                }
                IterState::Done => return None,
            })
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Bleh;
        #[test]
        fn bleh_into_iter() {
            let bleh = Bleh {
                first: 12,
                second: 99,
                third: 2,
            };
            assert_eq!(bleh.into_iter().collect::<Vec<_>>(), vec![12, 99, 2]);
        }
    }
}


