pub mod algorithm;
pub mod data_structure;
pub mod graph;
pub mod io;
pub mod math;

trait Int:
    Copy
    + Clone
    + PartialEq
    + Eq
    + std::ops::Add<Output = Self>
    + std::ops::AddAssign
    + std::ops::Sub<Output = Self>
    + std::ops::SubAssign
    + std::ops::Mul<Output = Self>
    + std::ops::MulAssign
    + std::ops::Div<Output = Self>
    + std::ops::DivAssign
    + std::ops::Rem<Output = Self>
    + std::ops::RemAssign
    + std::ops::Shl<Output = Self>
    + std::ops::Shr<Output = Self>
    + From<u8>
    + std::cmp::PartialOrd
    + std::cmp::Ord
{
}
impl Int for i16 {}
impl Int for i32 {}
impl Int for i64 {}
impl Int for i128 {}

trait UInt:
    Copy
    + Clone
    + PartialEq
    + Eq
    + std::ops::Add<Output = Self>
    + std::ops::AddAssign
    + std::ops::Sub<Output = Self>
    + std::ops::SubAssign
    + std::ops::Mul<Output = Self>
    + std::ops::MulAssign
    + std::ops::Div<Output = Self>
    + std::ops::DivAssign
    + std::ops::Rem<Output = Self>
    + std::ops::RemAssign
    + std::ops::Shl<Output = Self>
    + std::ops::Shr<Output = Self>
    + From<u8>
    + std::cmp::PartialOrd
    + std::cmp::Ord
{
}
impl UInt for u8 {}
impl UInt for u16 {}
impl UInt for u32 {}
impl UInt for u64 {}
impl UInt for u128 {}
impl UInt for usize {}

trait AllInt:
    Copy
    + Clone
    + PartialEq
    + Eq
    + std::ops::Add<Output = Self>
    + std::ops::AddAssign
    + std::ops::Sub<Output = Self>
    + std::ops::SubAssign
    + std::ops::Mul<Output = Self>
    + std::ops::MulAssign
    + std::ops::Div<Output = Self>
    + std::ops::DivAssign
    + std::ops::Rem<Output = Self>
    + std::ops::RemAssign
    + std::ops::Shl<Output = Self>
    + std::ops::Shr<Output = Self>
    + From<u8>
    + std::cmp::PartialOrd
    + std::cmp::Ord
{
}
impl AllInt for i16 {}
impl AllInt for i32 {}
impl AllInt for i64 {}
impl AllInt for i128 {}
impl AllInt for u8 {}
impl AllInt for u16 {}
impl AllInt for u32 {}
impl AllInt for u64 {}
impl AllInt for u128 {}
impl AllInt for usize {}
