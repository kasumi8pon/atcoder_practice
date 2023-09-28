use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        mut a: [usize; k],
        l: [usize; q]
    }

    for num in l.iter() {
        if a[num - 1] != n {
            if num < &k {
                if a[num - 1] + 1 != a[*num] {
                    a[num - 1] += 1;
                }
            } else {
                a[num - 1] += 1;
            }
        }
    }

    println!("{}", a.iter().join(" "));
}
