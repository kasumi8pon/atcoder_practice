use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: Usize1,
        d: usize,
        t: [usize; n+1]
    }

    let mut result: isize = -1;

    for i in 1..n {
        if t[i] - t[i - 1] <= d {
            result = t[i] as isize;
            break;
        }
    }

    println!("{}", result);
}
