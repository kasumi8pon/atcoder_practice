use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    if s[n - 1] == 'o' {
        println!("Yes");
    } else {
        println!("No");
    }
}
