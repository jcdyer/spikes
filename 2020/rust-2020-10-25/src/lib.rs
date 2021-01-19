//! Can I use ? to do non-error-specific .into() calls?

#![allow(clippy::try_err)]

pub fn handler(input: &str) -> Result<u64, String> {
    Ok(input.parse::<u64>().map_err(|_| "yo")?)
}

pub fn to_u64(byte: u8) -> Result<(), u64> {
    Err(byte)?;
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(handler("27"), Ok(27));
        assert_eq!(handler("abc"), Err(String::from("yo")));
        assert_eq!(to_u64(24u8), Err(24u64));
    }
}
