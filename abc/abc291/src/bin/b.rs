use proconio::input;

fn main() {
    input! {
        n: usize,
        mut x: [usize; 5 * n]
    }

    x.sort();

    let available_scores = &x[n..5 * n - n];
    let total_score: usize = available_scores.iter().sum();
    let answer = total_score as f64 /(3.0 * n as f64);

    println!("{}", answer);
}
