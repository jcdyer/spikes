#![feature(pattern)]

use std::str::pattern::{Pattern, DoubleEndedSearcher, ReverseSearcher, Searcher, SearchStep};
struct Quote;

impl<'a> Pattern<'a> for Quote {
    type Searcher = QuoteSearcher<'a>;

    fn into_searcher(self, haystack: &'a str) -> Self::Searcher {
        QuoteSearcher {
            haystack,
            finger: 0,
            finger_back: haystack.len(),
        }
    }
}

static SQ: char = '\'';
static DQ: char = '"';

struct QuoteSearcher <'a> {
    haystack: &'a str,
    finger: usize,
    finger_back: usize,
}

unsafe impl<'a> Searcher<'a> for QuoteSearcher<'a> {
    fn haystack(&self) -> &'a str {
        self.haystack
    }

    fn next(&mut self) -> SearchStep {
        use std::str::pattern::SearchStep::*;

        let finger = self.finger;
        let slice = unsafe {self.haystack.get_unchecked(self.finger..self.finger_back) };
        match slice.chars().next() {
            Some(c) => {
                self.finger += c.len_utf8();
                if c == SQ || c == DQ {
                    Match(finger, self.finger)
                } else {
                    Reject(finger, self.finger)
                }
            }
            None => Done,
        }
    }
}

unsafe impl<'a> ReverseSearcher<'a> for QuoteSearcher<'a> {
    fn next_back(&mut self) -> SearchStep {
        use std::str::pattern::SearchStep::*;

        let finger_back = self.finger_back;
        let slice = unsafe { self.haystack.get_unchecked(self.finger..self.finger_back) };
        match slice.chars().next_back() {
            Some(c) => {
                self.finger_back -= c.len_utf8();
                if c == SQ || c == DQ {
                    Match(self.finger_back, finger_back)
                } else {
                    Reject(self.finger_back, finger_back)
                }
            }
            None => Done,
        }
    }
}

impl<'a> DoubleEndedSearcher<'a> for QuoteSearcher<'a> {}

fn main() {
    for s in &[
        "First",
        "'Second'",
        r#""Third""#,
    ] {
        println!("{}", s.trim_start_matches(Quote).trim_end_matches(Quote));
        println!("{}", s.trim_matches(Quote));
    }
    println!("This 'quote' is at {}", "This 'quote' is at 5".find(Quote).unwrap());
    println!("This 'quote' is at {}", "This 'quote' is at 11".rfind(Quote).unwrap());
}

#[test]
fn test() {
    assert_eq!("this".trim_matches(Quote), "this");
    assert_eq!("'this'".trim_matches(Quote), "this");
    assert_eq!(r#""this""#.trim_matches(Quote), "this");
    assert_eq!(r#"'''"this'""#.trim_matches(Quote), "this");
    assert_eq!(r#"'''"this'""#.trim_matches(Quote), "this");
    assert_eq!(r"this 'quote' is at 5".find(Quote), Some(5));
    assert_eq!(r"this 'quote' is at 11".rfind(Quote), Some(11));
}
