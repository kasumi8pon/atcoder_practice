use proconio::input;

fn main() {
    input! {
        l1: usize,
        r1: usize,
        l2: usize,
        r2: usize
    }

    let l = l1.max(l2);
    let r = r1.min(r2);

    if r > l {
        println!("{}", r - l);
    } else {
        println!("{}", 0);
    }

}
