use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: u32,
        a: [i32; n]
    }

    let mut result = Vec::new();

    for i in a {
        if i % 2 == 0 {
            result.push(i);
        }
    }

    println!("{}", result.iter().join(" "))
}
