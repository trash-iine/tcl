#![allow(dead_code)]
extern crate num_traits;
use cargo_snippet::snippet;

#[snippet(name = "def_mod_pow", prefix = "extern crate num_traits;")]
fn mod_pow<T>(mut base: T, mut exp: T, modules: T) -> T
where
    T: num_traits::PrimInt,
{
    if modules == T::from(1).unwrap() {
        return T::from(0).unwrap();
    }
    let mut result = T::from(1).unwrap();
    base = base % modules;
    while exp > T::from(0).unwrap() {
        if exp % T::from(2).unwrap() == T::from(1).unwrap() {
            result = result * base % modules;
        }
        exp = exp >> 1;
        base = base * base % modules
    }
    result
}

#[snippet(name = "def_fast_pow", prefix = "extern crate num_traits;")]
fn fast_pow<T>(mut base: T, mut exp: T) -> T
where
    T: num_traits::PrimInt,
{
    let mut result = T::from(1).unwrap();
    while exp > T::from(0).unwrap() {
        if exp % T::from(2).unwrap() == T::from(1).unwrap() {
            result = result * base;
        }
        exp = exp >> 1;
        base = base * base;
    }
    result
}

#[test]
fn test_mod_pow() {
    assert_eq!(mod_pow(3 as i8, 4 as i8, 4 as i8), 1 as i8);
    assert_eq!(mod_pow(3 as i16, 5 as i16, 4 as i16), 3 as i16);
}
