#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(sieve(7), [2, 3, 5]);
    }

    #[test]
    fn it_works_2() {
        assert_eq!(sieve(15), [2, 3, 5, 7, 11, 13]);
    }
}


/// Find all prime numbers less than `n`.
pub fn sieve(n: u32) -> Vec<u32> {
    let mut primes: Vec<u32> = Vec::new();

    for i in 2..n {
        let mut is_prime = true;

        for k in 2..i {
                if i%k == 0 {
                    is_prime = false;
                }
        }

        if is_prime {
            primes.push(i);
        }
    }

    primes
}
