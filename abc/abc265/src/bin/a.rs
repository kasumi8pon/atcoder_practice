use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
        n: usize
    }

    if y < x * 3 {
        println!("{}", y * (n / 3) + x * (n % 3));
    } else {
        println!("{}", x * n);
    }
}
