The Sieve of Eratosthenes is used to find all primes below some given number (n), and is an efficient way to find small primes.

Iterate through the numbers from 2 to n. For each number i:

  - If i has been crossed-out from previous iterations, skip it.
  - If i isn't crossed-out yet, then it is prime. Cross-out all multiples of i from i*i to n. These are non-prime.

Head over to the Wikipedia page for a more detailed description and some zesty examples.

Your function will take a number, n, and return a list of all prime numbers less than n. (Notice that you can't use a Rust array in this case, since you don't know the length at compile time.)

```rust
/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
    // TODO
    unimplemented!();
}
```
