use std::collections::HashSet;


/// Computes the sum of all elements in the input i32 slice named `slice`
pub fn sum(slice: &[i32]) -> i32 {
    let mut sum = 0;
    for v in slice {
        sum += v;
    }
    sum
}

/// Deduplicates items in the input vector `vs`. Produces a vector containing
/// the first instance of each distinct element of `vs`, preserving the
/// original order.
pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    let mut set = HashSet::new();
    let mut new_vec: Vec<i32> = Vec::new();

    for v in vs {
        if set.insert(*v) {
            new_vec.push(*v);
        }
    }
    new_vec
}

/// Filters a vector `vs` using a predicate `pred` (a function from `i32` to
/// `bool`). Returns a new vector containing only elements that satisfy `pred`.
pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
    let mut new_vec: Vec<i32> = Vec::new();

    for v in vs {
        if pred(*v) {
            new_vec.push(*v);
        }
    }

    new_vec
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_sum() {
        let a: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(sum(&a), 15);
    }


    #[test]
    fn it_works_dedup() {
        let a = vec![1, 2, 3, 4, 5];
        let b = vec![1, 1, 1,1,1,1, 2, 3, 4, 5];
        assert!(dedup(&a) == dedup(&b));
    }

    #[test]
    fn it_works_dedup_2() {
        let a = vec![1, 2, 2, 2, 2];
        let b = vec![1, 2];
        assert!(dedup(&a) == b);
    }


    #[test]
    fn it_works_filter() {
        let greater_than_2 = |x: i32| -> bool {
            x > 2
        };

        let a = vec![1, 2, 5, 6, 7];
        let b = vec![5, 6, 7];
        assert!(filter(&a, &greater_than_2) == b);
    }
}
