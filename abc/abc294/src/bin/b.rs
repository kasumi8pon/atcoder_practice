use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }

    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 0 {
                print!(".");
            } else {
                print!("{}", (b'A' + (a[i][j] - 1) as u8) as char);
            }
        }
        println!("");
    }
}
