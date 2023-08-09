use proconio::input;

fn main() {
    input! {
        n: usize
    }

    if n == 2 || n == 3 || n == 4 {
        println!("No");
    } else {
        println!("Yes");
    }
}
