use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        a: Chars,
        b: Chars
    }

    let a_digit = a.len();
    let b_digit = b.len();
    let digits = vec![a_digit, b_digit];
    let min_digit = *digits.iter().min().unwrap();

    for i in 0..min_digit {
        if (a[a_digit - 1 - i] as u32 - '0' as u32) + (b[b_digit - 1 - i] as u32 - '0' as u32) > 9 {
            println!("Hard");
            return;
        }
    }

    println!("Easy");
}
