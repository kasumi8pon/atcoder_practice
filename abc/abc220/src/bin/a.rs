use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    }

    let min_multiple;

    if a % c == 0 {
        min_multiple = a;
    } else {
        min_multiple = c * ((a / c) + 1);
    }

    if min_multiple <= b {
        println!("{}", min_multiple);
    } else {
        println!("-1");
    }
}
