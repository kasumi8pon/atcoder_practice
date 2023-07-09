use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [String; n]
    }

    let mut a: Vec<Vec<char>> = a.iter().map(|s| s.chars().collect()).collect();

    let right_top = a[0][n - 1];
    let right_bottom = a[n - 1][n - 1];
    let left_bottom = a[n - 1][0];

    for i in (1..n).rev() {
        a[0][i] = a[0][i - 1]
    }

    for i in (2..n).rev() {
        a[i][n - 1] = a[i - 1][n - 1]
    }

    a[1][n - 1] = right_top;

    for i in 0..n-2 {
        a[n - 1][i] = a[n - 1][i + 1]
    }

    a[n - 1][n - 2] = right_bottom;

    for i in 0..n - 2 {
        a[i][0] = a[i + 1][0]
    }

    a[n - 2][0] = left_bottom;

    for i in 0..n {
        for j in 0..n {
            print!("{}", a[i][j]);
        }
        println!();
    }
}
