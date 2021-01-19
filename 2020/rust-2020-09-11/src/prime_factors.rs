pub fn prime_factors(mut val: u64, sieve: impl Iterator<Item=u64>) -> Vec<u64> {
    let mut factors = Vec::new();
    for prime in sieve {
        if val <= 1 {
            break
        }
        while val % prime == 0 { // while that prime is a factor of the number
            factors.push(prime);
            val /= prime;
        }
    }
    factors
}

#[test]
fn test_prime_factors() {
    let sieve = super::Sieve::with_capacity(24);
    assert_eq!(&prime_factors(2444, sieve.into_iter()), &[2, 2, 13, 47])
}