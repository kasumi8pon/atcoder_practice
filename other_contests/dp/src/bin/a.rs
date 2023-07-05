use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [i32; n]
    }

    let mut dp = Vec::new();

    for i in 0..n {
        if i == 0 {
            dp.push(0);
        } else if i == 1 {
            dp.push((h[i] - h[i - 1]).abs());
        } else {
            dp.push(std::cmp::min(
                dp[i - 1] + (h[i] - h[i - 1]).abs(),
                dp[i - 2] + (h[i] - h[i - 2]).abs(),
            ));
        }
    }

    println!("{}", dp[n - 1]);
}
