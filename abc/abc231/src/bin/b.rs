use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: [String; n]
    }

    let mut votes = HashMap::new();
    for name in s.iter() {
        *votes.entry(name).or_insert(0) += 1;
    }

    let answer = votes.iter().max_by_key(|&(_, v)| v).unwrap().0;

    println!("{}", answer);
}
