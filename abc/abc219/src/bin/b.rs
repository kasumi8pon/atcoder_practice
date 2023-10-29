use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s1: String,
        s2: String,
        s3: String,
        t: Chars
    }

    let strings = vec![s1, s2, s3];

    for i in t {
        let index = i.to_digit(10).unwrap() as usize;
        print!("{}", strings[index - 1])
    }
}
