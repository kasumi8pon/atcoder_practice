use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let alphabets: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    println!("{}", alphabets[n - 97]);
}
