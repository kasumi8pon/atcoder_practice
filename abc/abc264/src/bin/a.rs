use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        l: Usize1,
        r: Usize1
    }

    let atcoder = vec!["a", "t", "c", "o", "d", "e", "r"];

    println!("{}", atcoder[l..=r].join(""));
}
