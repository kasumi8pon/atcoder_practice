use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        t: Chars
    }

    let mut position = (0, 0);
    let mut direction = (1, 0);

    for i in 0..n {
        if t[i] == 'S' {
            position.0 += direction.0;
            position.1 += direction.1;
        } else {
            if direction == (1, 0) {
                direction = (0, -1);
            } else if direction == (0, -1) {
                direction = (-1, 0);
            } else if direction == (-1, 0) {
                direction = (0, 1);
            } else {
                direction = (1, 0);
            }
        }
    }

    println!("{} {}", position.0, position.1);
}
