use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let mut result = 0;

    s.iter().for_each(|&c|{
        if c == 'v' {
            result += 1
        } else if c == 'w' {
            result += 2
        }
    });

    println!("{}", result);
}
