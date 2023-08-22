use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let length = s.len();

    if length >= 3 && s[length - 3] == 'i' && s[length - 2] == 's' && s[length - 1] == 't' {
        println!("{}", "ist");
    } else if s[length - 2] == 'e' && s[length - 1] == 'r' {
        println!("{}", "er");
    }
}
