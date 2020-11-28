#![allow(dead_code)]
extern crate num_traits;
use cargo_snippet::snippet;

#[snippet(name = "@binary_search", prefix = "extern crate num_traits;")]
fn binary_search<T, F>(start: T, end: T, func: F) -> Result<T, T>
where
    T: num_traits::PrimInt,
    F: Fn(T) -> std::cmp::Ordering,
{
    let mut range = (start, end);
    while range.0 < range.1 {
        let mid = (start + end) / T::from(2).unwrap();
        let result = func(mid);
        match result {
            std::cmp::Ordering::Less => {
                range.0 = mid;
            },
            std::cmp::Ordering::Equal => {
                return Ok(mid);
            },
            std::cmp::Ordering::Greater => {
                range.1 = mid - T::from(1).unwrap();
            },
        }
    }

    return Err(range.0);
}
