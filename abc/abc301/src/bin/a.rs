use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let condition = ((n as f64) / 2.0).ceil() as i32;

    let mut t_count = 0;
    let mut a_count = 0;

    for i in 0..n {
        if s[i] == 'T' {
            t_count += 1
        } else if s[i] == 'A' {
            a_count += 1
        }

        if t_count == condition {
            println!("T");
            break;
        } else if a_count == condition {
            println!("A");
            break;
        }
    }
}
