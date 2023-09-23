use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n]
    }

    let mut answer = "Yes";

    for i in 0..n {
        if ['H', 'D', 'C', 'S'].iter().any(|&x| x == s[i][0]) &&
            ['A', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K'].iter().any(|&x| x == s[i][1]) {
            continue;
            } else {
                answer = "No";
                break;
            }
    }

    let size = s.len();
    s.sort();
    s.dedup();

    if s.len() != size {
        answer = "No";
    }

    println!("{}", answer);
}
