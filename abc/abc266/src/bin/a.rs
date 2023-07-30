use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let mid_index = (s.len() + 1) / 2;
    println!("{}", s[mid_index - 1]);
}
