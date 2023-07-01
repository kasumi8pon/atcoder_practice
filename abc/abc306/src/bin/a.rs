use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    for i in 0..n {
        print!("{}{}", s[i], s[i]);
    }
}
