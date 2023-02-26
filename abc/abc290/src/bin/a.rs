use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [Usize1; m],
    }

    let mut sum = 0;

    for index in b {
        sum += a[index];
    }

    println!("{}", sum);
}
