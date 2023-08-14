use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize
    }

    let answer;

    if x >= y {
        answer = 0
    } else {
        answer = (y - x + 9) / 10
    }

    println!("{}", answer);
}
