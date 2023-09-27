use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        a: [Chars; n]
    }

    for i in 0..n {
        for j in 0..n {
            if i == j {
                if a[i][j] != '-' {
                    println!("incorrect");
                    return;
                }
            } else if i < j {
                if a[i][j] == 'W' {
                    if a[j][i] != 'L' {
                        println!("incorrect");
                        return;
                    }
                } else if a[i][j] == 'L' {
                    if a[j][i] != 'W' {
                        println!("incorrect");
                        return;
                    }
                } else {
                    if a[j][i] != 'D' {
                        println!("incorrect");
                        return;
                    }
                }
            }
        }
    }

    println!("correct");
}
