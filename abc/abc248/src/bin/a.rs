use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
    }

    let answer: Vec<char> = "0123456789".chars().filter(|c| !s.contains(c)).collect();
    println!("{}", answer[0]);
}
