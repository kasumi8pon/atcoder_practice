use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n]
    }

    let max = p[1..n].iter().max().unwrap_or(&0);

    if p[0] > *max {
        println!("{}", 0);
    } else {
        println!("{}", max - p[0] + 1);
    }
}
