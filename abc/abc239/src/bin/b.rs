use proconio::input;

fn main() {
    input! {
        x: i64
    }

    let answer;

    if x >= 0 {
        answer = x / 10;
    } else {
        answer = (x - 9) / 10;
    }

    println!("{}", answer);
}
