use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n]
    }

    let mut reversed_s = s.clone();
    reversed_s.reverse();

    for i in 0..n {
        println!("{}", reversed_s[i]);
    }
}
