use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut sorted_a = a.clone();
    sorted_a.sort();
    sorted_a.reverse();
    let score = sorted_a[1];

    for i in 0..n {
        if a[i] == score {
            println!("{}", i + 1);
            return;
        }
    }
}
