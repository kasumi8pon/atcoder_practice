use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        s: [isize; n]
    }

    let mut counter = 0;
    let mut answers = vec![0; n];

    for i in 0..n {
        let a = s[i] - counter;
        answers[i] = a;
        counter += a;
    }

    println!("{}", answers.iter().join(" "));
}
