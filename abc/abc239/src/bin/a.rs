use proconio::input;

fn main() {
    input! {
        h: f64
    }

    let answer = (h * (12800000 as f64 + h)).sqrt();

    println!("{}", answer);
}
