use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let a = n / 100;
    let b = n / 10 % 10;
    let c = n % 10;

    let answer = n + b * 100 + c * 10 + a * 1 + c * 100 + a * 10 + b * 1;

    println!("{}", answer);
}
