#[derive(Default, Debug)]
pub struct Interner {
    vec: Vec<String>,
    // Invariant: sorted
    set: Vec<usize>,
}

impl Interner {
    pub fn intern(&mut self, s: &str) -> usize {
        let idx = self
            .set
            .binary_search_by(|&idx| self.vec[idx].as_str().cmp(s));

        dbg!(
            s,
            match idx {
                Ok(idx) => {
                    self.set[idx]
                }
                Err(idx) => {
                    let res = self.vec.len();
                    self.vec.push(s.to_string());
                    self.set.insert(idx, res);
                    res
                }
            }
        )
        .1
    }
    pub fn lookup(&self, i: usize) -> &str {
        &self.vec[i]
    }
}

#[cfg(test)]
mod tests {
    use super::Interner;

    #[test]
    fn it_works() {
        let mut int = Interner::default();
        let text = "let us not forget the sacrifice of the angry men who sacrifice other angry men who let us sacrifice";
        for word in text.split_whitespace() {
            int.intern(word);
        }
        #[rustfmt::skip]
        assert_eq!(
            int.vec,
            ["let", "us", "not", "forget", "the", "sacrifice", "of", "angry", "men", "who", "other"],
        );

        assert_eq!(int.set, [7, 3, 0, 8, 2, 6, 10, 5, 4, 1, 9]);
    }
}

// async unsafe extern pub fn f() {}
// async extern pub unsafe fn g() {}
// extern async pub unsafe fn h() {}
// extern pub async unsafe fn h() {}
// pub async extern unsafe fn i() {}
// pub unsafe async extern fn j() {}
pub async unsafe extern fn k() {}
// unsafe async extern pub fn l() {}
// unsafe pub async extern fn m() {}

// pub const unsafe fn m() {}
// pub async unsafe extern "C" fn n() {}