use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars
    }

    let length = s.len() / 2;

    for i in 1..=length {
        let tmp = s[2 * i - 2];
        s[2 * i - 2] = s[2 * i - 1];
        s[2 * i - 1] = tmp;
    }

    let result: String = s.into_iter().collect();

    println!("{}", result);
}
