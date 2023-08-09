use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }

    if a == 1 && b == 10 || b - a == 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
