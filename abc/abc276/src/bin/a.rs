use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars
    }

    s.reverse();

    let min_position = s.iter().position(|&char| char == 'a');
    match min_position {
        Some(i) => println!("{}", s.len() - i),
        None => println!("{}", -1)
    }
}
