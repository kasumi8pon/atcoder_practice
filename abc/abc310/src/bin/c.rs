use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n]
    }

    for chars in &mut s {
        let mut reversed_chars = chars.clone();
        reversed_chars.reverse();
        if &reversed_chars < chars {
            *chars = reversed_chars;
        }
    }

    s.sort();
    s.dedup();

    println!("{}", s.len());
}
