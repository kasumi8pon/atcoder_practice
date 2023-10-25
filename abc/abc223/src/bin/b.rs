use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars
    }

    let mut s_min = s.iter().collect();
    let mut s_max = s.iter().collect();

    for _ in 0..s.len() {
        let first_char = s.remove(0);
        s.push(first_char);

        let shifted_s = s.iter().collect::<String>();

        if shifted_s < s_min {
            s_min = shifted_s.clone();
        }

        if shifted_s > s_max {
            s_max = shifted_s.clone();
        }
    }

    println!("{}\n{}", s_min, s_max);
}
