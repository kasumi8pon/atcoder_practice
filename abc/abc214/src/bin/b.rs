use proconio::input;

fn main() {
    input! {
        s: usize,
        t: usize
    }

    let mut answer = 0;

    for i in 0..=s {
        for j in 0..=s {
            for k in 0..=s {
                if i + j + k <= s && i * j * k <= t {
                    answer += 1;
                }
            }
        }
    }

    println!("{}", answer);
}
