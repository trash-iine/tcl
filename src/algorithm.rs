#![allow(dead_code)]

use super::*;
use cargo_snippet::snippet;

#[snippet(name = "@binary_search")]
fn binary_search<T, F>(lower: T, upper: T, func: F) -> Result<T, (T, T)>
where
    T: AllInt,
    F: Fn(T) -> std::cmp::Ordering,
{
    let mut range = (lower, upper);

    while range.0 < range.1 {
        let mid = (range.0 + range.1) / T::from(2);
        if mid == range.0 || mid == range.1 {
            break;
        }

        let result = func(mid);
        match result {
            std::cmp::Ordering::Less => {
                range.0 = mid;
            }
            std::cmp::Ordering::Equal => {
                return Ok(mid);
            }
            std::cmp::Ordering::Greater => {
                range.1 = mid;
            }
        }
    }

    return Err(range);
}

#[test]
fn test_binary_search() {
    let mut last = 1;
    let mut last2 = 1;

    for i in 2..10_000u64 {
        let i2 = i * i;

        for j in (last2 + 1)..i {
            assert_eq!(binary_search(1, j, |x| (x * x).cmp(&j)), Err((last, i)));
        }

        assert_eq!(binary_search(1, i2, |x| (x * x).cmp(&i2)), Ok(i));

        last = i;
        last2 = i2;
    }
}
