use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        d: [usize; n]
    }

    let min = d.iter().min().unwrap();

    if min + q <= p {
        println!("{}", (min + q));
    } else {
        println!("{}", p);
    }
}
