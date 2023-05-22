use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let mut o = false;
    let mut x = false;

    for i in 0..n {
        if s[i] == 'x' {
            x = true;
            break;
        }

        if s[i] == 'o' {
            o = true;
        }
    }

    if o && !x {
        println!("Yes");
    } else {
        println!("No");
    }
}
