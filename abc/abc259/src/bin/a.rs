use proconio::input;

fn main() {
    input! {
        _n: usize,
        m: usize,
        x: usize,
        t: usize,
        d: usize
    }

    if m >= x {
        println!("{}", t);
    } else {
        println!("{}", t - (x - m) * d);
    }
}
