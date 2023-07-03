use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; 7 * n],
    }

    let mut sum = 0;

    for i in 0..7 * n {
        if i % 7 == 0 {
            sum = 0;
        }

        sum += a[i];

        if i % 7 == 6 {
            print!("{} ", sum);
        }
    }
}
