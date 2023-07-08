use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        _w: usize,
        s: [Chars; h]
    }

    let count = s.iter().flatten().filter(|&c| *c == '#').count();
    println!("{}", count);
}
