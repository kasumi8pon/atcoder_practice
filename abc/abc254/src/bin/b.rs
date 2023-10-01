use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize
    }

    let mut numbers = vec![vec![]; n];

    for i in 0..n {
        for j in 0..=i {
            if j == 0 || j == i {
                numbers[i].push(1);
            } else {
                let x = numbers[i - 1][j - 1];
                let y = numbers[i - 1][j];
                numbers[i].push(x + y);
            }
        }

        println!("{}", numbers[i].iter().join(" "));
    }
}
