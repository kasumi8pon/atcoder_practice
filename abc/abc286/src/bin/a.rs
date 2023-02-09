use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        p: Usize1,
        q: usize,
        r: Usize1,
        _s: usize,
        mut a:[usize;n]
    };

    for i in p..q {
        // https://doc.rust-lang.org/std/primitive.slice.html#method.swap
        a.swap(i, i + r - p);
    }

    for x in a {
        print!("{} ", x);
    }
}
