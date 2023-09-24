use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: [Chars; 10]
    }

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;

    for i in 0..10 {
        for j in 0..10 {
            if s[i][j] == '#' && a == 0 {
                a = i + 1;
                c = j + 1;
            }
            if s[i][j] == '#' {
                b = i + 1;
                d = j + 1;
            }
        }
    }

    println!("{} {}", a, b);
    println!("{} {}", c, d);
}
