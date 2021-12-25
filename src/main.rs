#![allow(dead_code, unused_macros, unused_imports)]
use std::{collections::*, vec};

/// input macro from https://qiita.com/tanakh/items/1ba42c7ca36cd29d0ac8
macro_rules ! read_value {($ next : expr , ($ ($ t : tt ) ,* ) ) => {($ (read_value ! ($ next , $ t ) ) ,* ) } ; ($ next : expr , [$ t : tt ; $ len : expr ] ) => {(0 ..$ len ) . map (| _ | read_value ! ($ next , $ t ) ) . collect ::< Vec < _ >> () } ; ($ next : expr , chars ) => {read_value ! ($ next , String ) . chars () . collect ::< Vec < char >> () } ; ($ next : expr , usize1 ) => {read_value ! ($ next , usize ) - 1 } ; ($ next : expr , $ t : ty ) => {$ next () . parse ::<$ t > () . expect ("Parse error" ) } ; }
macro_rules ! input_inner {($ next : expr ) => {} ; ($ next : expr , ) => {} ; ($ next : expr , $ var : ident : $ t : tt $ ($ r : tt ) * ) => {let $ var = read_value ! ($ next , $ t ) ; input_inner ! {$ next $ ($ r ) * } } ; }
macro_rules ! input {(source = $ s : expr , $ ($ r : tt ) * ) => {let mut iter = $ s . split_whitespace () ; let mut next = || {iter . next () . unwrap () } ; input_inner ! {next , $ ($ r ) * } } ; ($ ($ r : tt ) * ) => {let stdin = std :: io :: stdin () ; let mut bytes = std :: io :: Read :: bytes (std :: io :: BufReader :: new (stdin . lock () ) ) ; let mut next = move || -> String {bytes . by_ref () . map (| r | r . unwrap () as char ) . skip_while (| c | c . is_whitespace () ) . take_while (| c |! c . is_whitespace () ) . collect () } ; input_inner ! {next , $ ($ r ) * } } ; }

macro_rules ! input_one_line {(source = $ s : expr , $ ($ r : tt ) * ) => {let mut iter = $ s . split_whitespace () ; let mut next = || {iter . next () . unwrap () } ; input_inner ! {next , $ ($ r ) * } } ; ($ ($ r : tt ) * ) => {let mut words = String :: new () ; std :: io :: stdin () . read_line (& mut words ) . ok () ; let mut bytes = words . bytes () ; let mut next = move || -> String {bytes . by_ref () . map (| r | r as char ) . skip_while (| c | c . is_whitespace () ) . take_while (| c |! c . is_whitespace () ) . collect () } ; input_inner ! {next , $ ($ r ) * } } ; }
macro_rules ! rough_print {($ x : expr $ (, $ s : expr ) * ) => {print ! ("{:?}" , ($ x ) ) ; $ (print ! (", {:?}" , ($ s ) ) ; ) * println ! ("" ) ; } ; }

const BIG_PRIME: u64 = 1_000_000_007;
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

fn solve() {
    input_one_line!(_t: u64);
    for _ in 0.._t {}
}

fn main() {
    std::thread::Builder::new()
        .name("solve".into())
        .stack_size(256 * 1024 * 1024)
        .spawn(solve)
        .unwrap()
        .join()
        .unwrap();
}

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
