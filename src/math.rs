#![allow(dead_code)]

use super::*;
use cargo_snippet::snippet;

#[snippet(name = "@mod_pow")]
fn mod_pow<T>(mut base: T, mut exp: T, modules: T) -> T
where
    T: UInt,
{
    if modules == T::from(1) {
        return T::from(0);
    }
    let mut result = T::from(1);
    base = base % modules;
    while exp > T::from(0) {
        if exp % T::from(2) == T::from(1) {
            result = result * base % modules;
        }
        exp = exp >> T::from(1);
        base = base * base % modules
    }
    result
}

#[snippet(name = "@fast_pow")]
fn fast_pow<T>(mut base: T, mut exp: T) -> T
where
    T: UInt,
{
    let mut result = T::from(1);
    while exp > T::from(0) {
        if exp % T::from(2) == T::from(1) {
            result = result * base;
        }
        exp = exp >> T::from(1);
        base = base * base;
    }
    result
}

#[snippet(name = "@gcd")]
fn gcd<T>(a: T, b: T) -> T
where
    T: AllInt,
{
    if b == T::from(0) {
        a
    } else {
        gcd(b, a % b)
    }
}

#[snippet(name = "@gcd_list", include = "@gcd")]
fn gcd_list<T>(list: &[T]) -> T
where
    T: AllInt,
{
    list.iter().fold(list[0], |a, &b| gcd(a, b))
}

#[snippet(name = "@ext_gcd")]
/// CAUTION: This function cannot be applied to unsigned integer!
fn ext_gcd<T>(a: T, b: T, x: &mut T, y: &mut T) -> T
where
    T: Int,
{
    if b == T::from(0) {
        *x = T::from(1);
        *y = T::from(0);
        return a;
    } else {
        let d = ext_gcd(b, a % b, y, x);
        *y = (*y) - a / b * (*x);
        return d;
    }
}

#[snippet(name = "@is_prime", include = "@mod_pow")]
fn is_prime<T>(n: T) -> bool
where
    T: UInt,
{
    if n <= T::from(1) {
        return false;
    }

    if n == T::from(2) || n == T::from(7) || n == T::from(61) {
        return true;
    }

    if n % T::from(2) == T::from(0) {
        return false;
    }

    let mut d = n - T::from(1);
    while d % T::from(2) == T::from(0) {
        d /= T::from(2);
    }

    for a in vec![T::from(2), T::from(7), T::from(61)] {
        let mut t = d;
        let mut y = mod_pow(a, t, n);

        while t != n - T::from(1) && y != T::from(1) && y != n - T::from(1) {
            y = (y * y) % n;
            t = t << T::from(1);
        }

        if y != n - T::from(1) && t % T::from(2) == T::from(0) {
            return false;
        }
    }

    return true;
}

#[test]
fn test_mod_pow() {
    assert_eq!(mod_pow(3u8, 4u8, 4u8), 1u8);
    assert_eq!(mod_pow(3u16, 5u16, 4u16), 3u16);
}
