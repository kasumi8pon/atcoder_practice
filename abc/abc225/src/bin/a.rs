use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars
    }

    s.sort();
    s.dedup();

    let number = s.len();

    let answer;

    if number == 3 {
        answer = 6;
    } else if number == 2 {
        answer = 3;
    } else {
        answer = 1;
    }

    println!("{}", answer);
}
