use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize
    }

    let alphabets: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();

    println!("{}", alphabets[(x - 1) / n]);
}
