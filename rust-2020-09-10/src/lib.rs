use std::convert::TryInto;

pub fn str_to_u64(mut value: String) -> Option<u64> {
    let mut buf = [0; 8];

    if value.len() > 8 {
        None
    } else {
        buf[..value.len()].copy_from_slice(&value.into_bytes());
        Some(u64::from_le_bytes(buf))
    }
}

pub fn u64_as_str<'a>(orig: &'a u64) -> Option<&'a str> {
    let slice: &[u8; 8] = unsafe { std::mem::transmute(orig) };
    std::str::from_utf8(&*slice)
        .map(|st| st.trim_end_matches('\0'))
        .ok()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(str_to_u64("Cliff D.".into()).unwrap(), 3333825248337947715);
        assert_eq!(str_to_u64("Dyer".into()).unwrap(), 1919252804);
        assert_eq!(u64_as_str(&3333825248337947715).unwrap(), "Cliff D.");
        assert_eq!(u64_as_str(&1919252804).unwrap(), "Dyer");
    }

}
