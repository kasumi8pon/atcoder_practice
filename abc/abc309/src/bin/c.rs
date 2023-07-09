use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut ab: [(usize, usize); n]
    }

    let mut sum = ab.iter().map(|(_, b)| b).sum::<usize>();

    ab.sort_by(|a, b| a.0.cmp(&b.0));

    if sum <= k {
        println!("1");
    } else {
        for i in 0..n {
            sum -= ab[i].1;

            if sum <= k {
                println!("{}", ab[i].0 + 1);
                return;
            };
        }
    }

}
