#![allow(unused_macros, dead_code)]
use cargo_snippet::snippet;

#[snippet(name = "@input")]
/// input macro from https://qiita.com/tanakh/items/1ba42c7ca36cd29d0ac8
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

#[snippet(name = "@input")]
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

#[snippet(name = "@input")]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

#[snippet(name = "@input")]
macro_rules! input_one_line {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let mut words = String::new();
        std::io::stdin().read_line(&mut words).ok();
        let mut bytes = words.bytes();
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

#[snippet(name = "@print_with_yes_no")]
fn print_with_yes_no(b: bool) {
    if b {
        println!("Yes");
    } else {
        println!("No");
    }
}

#[snippet(name = "@rough_print")]
macro_rules! rough_print {
    ($x:expr $(, $s:expr)*) => {
        print!("{:?}", ($x));
        $( print!(", {:?}", ($s)); )*
        println!("");
    };
}
