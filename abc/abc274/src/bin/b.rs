use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h]
    }

    for i in 0..w {
        let mut count = 0;

        for j in 0..h {
            if c[j][i] == '#' {
                count += 1;
            }
        }

        println!("{}", count);
    }
}
