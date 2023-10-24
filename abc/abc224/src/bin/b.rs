use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[u64; w]; h]
    }

    for i1 in 0..h {
        for i2 in 0..h {
            if i1 >= i2 {
                continue;
            }
            for j1 in 0..w {
                for j2 in 0..w {
                    if j1 >= j2 {
                        continue;
                    }
                    if a[i1][j1] + a[i2][j2] > a[i2][j1] + a[i1][j2] {
                        println!("No");
                        return;
                    }
                }
            }
        }
    }

    println!("Yes");
}
