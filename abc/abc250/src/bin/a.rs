use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        r: usize,
        c: usize,
    }

    let mut answer = 0;

    if r + 1 <= h {
        answer += 1;
    }

    if c + 1 <= w {
        answer += 1;
    }

    if r - 1 >= 1 {
        answer += 1;
    }

    if c - 1 >= 1 {
        answer += 1;
    }

    println!("{}", answer);
}
