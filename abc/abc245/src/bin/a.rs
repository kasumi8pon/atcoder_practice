use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    let answer;

    if a < c {
        answer = "Takahashi"
    } else if a == c {
        if b <= d {
            answer = "Takahashi"
        } else {
            answer = "Aoki"
        }
    } else {
        answer = "Aoki"
    }

    println!("{}", answer);
}
