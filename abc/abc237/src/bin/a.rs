use proconio::input;

fn main() {
    input! {
        n: i64
    }

    if n >= -2_i64.pow(31) && n < 2_i64.pow(31) {
        println!("Yes");
    } else {
        println!("No");
    }
}
