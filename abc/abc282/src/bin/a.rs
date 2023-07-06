use proconio::input;

fn main() {
    input! {
        k: usize
    }

    let alphabets = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let result = &alphabets[0..k];

    println!("{}", result);
}
