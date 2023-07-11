use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        p: [usize; n]
    }

    let result = p.iter().position(|&i| i == x).unwrap() + 1;

    println!("{}", result);
}
