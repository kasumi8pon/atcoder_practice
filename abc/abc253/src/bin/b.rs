use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    }

    let mut x1 = -1;
    let mut y1 = -1;
    let mut x2 = -1;
    let mut y2 = -1;

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'o' {
                if x1 != -1 && y1 != -1 {
                    x2 = i as isize;
                    y2 = j as isize;
                } else {
                    x1 = i as isize;
                    y1 = j as isize;
                }
            }
        }
    }

    println!("{}", (x2 - x1).abs() + (y2 - y1).abs());
}
