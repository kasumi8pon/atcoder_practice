use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;

fn main() {
    input!{
        n: Usize1,
        s: Chars
    }

    let mut result = "Yes";

    for i in 0..n {
        if s[i] == s[i + 1] {
            result = "No";
            break;
        }
    }

    println!("{}", result);
}
