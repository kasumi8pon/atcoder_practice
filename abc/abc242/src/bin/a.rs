use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        c: f64,
        x: f64
    }

    let answer;

    if x <= a {
        answer = 1.0
    } else if x <= b {
        answer = c / (b - a)
    } else {
        answer = 0.0
    }

    println!("{}", answer);
}
