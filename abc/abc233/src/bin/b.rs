use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;

fn main() {
    input! {
        l: Usize1,
        r: Usize1,
        s: Chars
    }

    let length = s.len();

    for i in 0..length {
        if l <= i && i <= r {
            print!("{}", s[r + l - i]);
        } else {
            print!("{}", s[i]);
        }
    }
}
