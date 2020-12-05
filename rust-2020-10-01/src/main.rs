// xor distance
//
// This was a coding exercise I encountered in a job interview.

fn main() {
    println!("Hello, world!");
}


pub fn find_nearest(origin: u64, field: &[u64], n: usize) -> Vec<u64> {
    let mut field: Vec<_> = field.iter().map(|pt| (origin ^ pt, pt)).collect();
    field.sort();
    field.iter().take(n).map(|pair| *pair.1).collect()
}

#[test]
fn nearest_are_nearest() {
    let nearest_half = find_nearest(rand::random(), &std::iter::repeat_with(||rand::random()).take(256).collect::<Vec<_>>(), 128);
}
