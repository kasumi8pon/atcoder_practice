use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
        mut b: [usize; m]
    }

    let mut ok = 2 * 10_usize.pow(9) + 1;
    let mut ng = 0;

    while ok - ng > 1 {
        let mid = (ok + ng) / 2;

        let sellers = a.iter().filter(|x| **x <= mid).count();
        let buyers = b.iter().filter(|x| **x >= mid).count();

        if sellers >= buyers {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}
