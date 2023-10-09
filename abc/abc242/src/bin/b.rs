use proconio::input;
use proconio::marker::Chars;
use itertools::Itertools;

fn main() {
    input! {
        mut s: Chars
    }

    s.sort();

    println!("{}", s.iter().join(""));
}
