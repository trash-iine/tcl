extern crate num_traits;

fn mod_pow<T>(mut base: T, mut exp: T, modulus: T) -> T
where
    T: num_traits::PrimInt,
{
    if modulus == T::from(1).unwrap() {
        return T::from(0).unwrap();
    }
    let mut result = T::from(1).unwrap();
    base = base % modulus;
    while exp > T::from(0).unwrap() {
        if exp % T::from(2).unwrap() == T::from(1).unwrap() {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}
