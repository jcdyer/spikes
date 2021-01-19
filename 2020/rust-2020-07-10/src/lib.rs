use std::ops::Deref;
use std::ops::DerefMut;

#[repr(C, align(4096))]
pub struct AlignedBuf {
    data: [u8; 4096],
}

impl AlignedBuf {
    pub fn new(data: [u8; 4096]) -> AlignedBuf {
        AlignedBuf { data }
    }
}

impl Deref for AlignedBuf {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.data
    }
}

impl DerefMut for AlignedBuf {
    fn deref_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
}

#[cfg(test)]
mod tests {
    use super::AlignedBuf;

    #[test]
    fn is_aligned() {
        for i in 0..255 {
            let box_ab = Box::new(AlignedBuf::new([i; 4096]));
            let ab_ptr = box_ab.as_ptr();
            dbg!((i, ab_ptr as usize));
            assert_eq!(ab_ptr as usize % 4096, 0);
            //bufs.push(box_ab);
        }
    }

    #[test]
    fn is_buf() {
    }
}

