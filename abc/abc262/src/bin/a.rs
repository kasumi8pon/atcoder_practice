use proconio::input;

fn main() {
    input! {
        y: usize
    }

    let mut answer = y.clone();
    let rem = y % 4;

    if rem == 0 {
        answer += 2;
    } else if rem == 1 {
        answer += 1;
    } else if rem == 3 {
        answer += 3;
    }

    println!("{}", answer);
}
