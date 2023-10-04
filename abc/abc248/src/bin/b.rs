use proconio::input;

fn main() {
    input! {
        mut a: usize,
        b: usize,
        k: usize
    }

    let mut answer = 0;

    loop {
        if a >= b {
            break;
        }

        a *= k;
        answer += 1;
    }

    println!("{}", answer);
}
