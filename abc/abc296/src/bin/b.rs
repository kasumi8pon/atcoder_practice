use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: [Chars; 8]
    }

    let vertical = ["8", "7", "6", "5", "4", "3", "2", "1"];
    let horizontal = ["a", "b", "c", "d", "e", "f", "g", "h"];

    for i in 0..8 {
        for j in 0..8 {
            if s[i][j] == '*' {
                println!("{}{}", horizontal[j], vertical[i]);
            }
        }
    }
}
