use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let mut first_b = 9;
    let mut second_b = 9;
    let mut first_r = 9;
    let mut second_r = 9;
    let mut k = 9;

    for i in 0..8 {
        if s[i] == 'B' {
            if first_b == 9 {
                first_b = i;
            } else {
                second_b = i;
            }
        } else if s[i] == 'R' {
            if first_r == 9 {
                first_r = i;
            } else {
                second_r = i;
            }
        } else if s[i] == 'K' {
            k = i;
        }
    }

    if (first_b % 2 != second_b % 2) && first_r < k && k < second_r {
        println!("Yes");
    } else {
        println!("No");
    }
}
