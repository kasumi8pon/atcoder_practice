use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [[usize; n]; n],
        b: [[usize; n]; n],
    }

    let mut current = a.clone();

    for _ in 0..=3 {
        let mut rotated_a = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                rotated_a[i][j] = current[n - j - 1][i];
            }
        }

        let mut same = true;

        for i in 0..n {
            for j in 0..n {
                if rotated_a[i][j] == 1 && b[i][j] == 0 {
                    same = false;
                    break;
                }
            }
        }

        if same {
            println!("Yes");
            return;
        } else {
            current = rotated_a;
        }
    }

    println!("No");
}
