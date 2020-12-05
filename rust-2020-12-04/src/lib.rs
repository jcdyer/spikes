use std::io::{BufRead, Cursor};
fn main() {
    let x = std::io::BufReader::new(std::io::Cursor::new("a b c\nd e f\n"));
    let v = x
        .lines()
        .map(|l| l.unwrap())
        .map(|line| {
            line.split_whitespace()
                .map(|w| w.to_owned())
                .collect::<Vec<_>>()
        })
        .flatten();
    println!("{:?}", v)
}

pub trait WordsReader {
    fn read_word(&mut self, s: &mut String) -> std::io::Result<usize>;
    fn words(self) -> Words<Self> where Self: Sized;
}

impl<R: Sized + BufRead> WordsReader for R {
    fn read_word(&mut self, out: &mut String) -> std::io::Result<usize> {
        let inlen = out.len();
        loop {
            let m = self.fill_buf()?;
            if m.is_empty() {
                break;
            }
            match std::str::from_utf8(m) {
                Ok(s) => {
                    match s.find(char::is_whitespace) {
                        Some(mut n) => {
                            out.push_str(&s[..n]);
                            while let Some(c) = s[n..].chars().next() {
                                if c.is_whitespace() {
                                    out.push(c);
                                    n += 1;
                                }
                            }
                            self.consume(n);
                            break;

                        }
                        None => {
                            out.push_str(s);
                            let amt_read = s.len();
                            self.consume(amt_read);
                            continue;
                        }
                    }
                }
                Err(utf8_err) => {
                    panic!("Need to handle incomplete utf8")
                }
            };
        }
        Ok(out.len() - inlen)
    }

    fn words(self) -> Words<Self> {
        Words { buf: self }
    }
}

pub struct Words<T> {
    buf: T,
}

static WHITESPACE: &[char] = &[' ', '\t', '\n'];

impl<R> Iterator for Words<R>
where
    R: BufRead,
{
    type Item = std::io::Result<String>;
    fn next(&mut self) -> Option<std::io::Result<String>> {
        let mut word = String::new();
        match self.buf.read_word(&mut word) {
            Ok(0) => None,
            Ok(_) => {
                while word.ends_with(WHITESPACE) {
                    word.pop();
                }
                Some(Ok(word))
            }
            Err(err) => Some(Err(err)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn empty_reader() {
        let r = Cursor::new("");
        assert_eq!(ok_collect(r.words()), Vec::<String>::new());
    }

    #[test]
    fn one_word() {
        let r = Cursor::new("one");
        assert_eq!(ok_collect(r.words()), ["one"].into_iter().map(|&s| String::from(s)).collect::<Vec<_>>());
    }

    fn ok_collect(from: impl Iterator<Item=std::io::Result<String>>) -> Vec<String> {
        from.collect::<Result<Vec<String>, _>>().expect("errors in iterator")
    }

}