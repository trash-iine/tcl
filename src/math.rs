fn mod_pow<T>(mut base: T, mut exp: T, modulus: T) -> T
where
    T: std::ops::Mul<Output = T>
        + std::ops::Rem<Output = T>
        + PartialEq<T>
        + PartialOrd<T>
        + std::ops::Shr<Output = T>
        + From<u8>
        + Copy,
{
    if modulus == T::from(1) {
        return T::from(0);
    }
    let mut result = T::from(1);
    base = base % modulus;
    while exp > T::from(0) {
        if exp % T::from(2) == T::from(1) {
            result = result * base % modulus;
        }
        exp = exp >> T::from(1);
        base = base * base % modulus
    }
    result
}
