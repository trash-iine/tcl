extern crate num_traits;

#[snippet(name = "defmodpow", prefix = "extern crate num_traits;")]
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
