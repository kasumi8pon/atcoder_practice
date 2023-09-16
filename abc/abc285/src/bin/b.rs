use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    for i in 1..n {
        let mut answer = n - i;
        for l in 0..n - i {
            if s[l] == s[l + i] {
                answer = l;
                break;
            }
        }
        println!("{}", answer);
    }
}
