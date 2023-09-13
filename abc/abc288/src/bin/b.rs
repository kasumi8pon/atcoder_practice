use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut s: [String; n]
    }

    s[0..k].sort();

    for person in s[0..k].iter() {
        println!("{}", person);
    }
}
