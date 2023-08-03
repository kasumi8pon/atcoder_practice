use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let repetitions = 6 / s.len();

    for _ in 0..repetitions {
        for &c in &s {
            print!("{}", c);
        }
    }
}
