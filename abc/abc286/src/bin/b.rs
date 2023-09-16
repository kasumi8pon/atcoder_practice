use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    print!("{}", s[0]);

    for i in 1..n {
        if s[i - 1] == 'n' && s[i] == 'a' {
            print!("{}", 'y');
        }
        print!("{}", s[i]);
    }
}
