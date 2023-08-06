use proconio::input;
use proconio::marker::Chars;
use itertools::Itertools;

fn main() {
    input! {
        mut s: Chars
    }

    let mut answer = vec!['0'];
    answer.push(s[0]);
    answer.push(s[1]);
    answer.push(s[2]);

    println!("{}", answer.iter().join(""));
}
