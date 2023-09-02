use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize; n]; m],
    }

    let mut counted = vec![vec![false; n]; n];

    for i in 0..m {
        for j in 0..(n-1) {
            let x;
            let y;

            if a[i][j] < a[i][j+1] {
                x = a[i][j];
                y = a[i][j+1];
            } else {
                x = a[i][j+1];
                y = a[i][j];
            }

            counted[x - 1][y - 1] = true;
        }
    }

    let mut count = 0;

    for i in 0..n {
        count += counted[i].iter().filter(|&&x| x == true).count();
    }

    println!("{}", ((n * (n - 1)) / 2) - count);
}
