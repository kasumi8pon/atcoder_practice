use proconio::input;

fn main() {
    input! {
        n: u64
    }

    for i in 0.. {
        if 2u64.pow(i) > n {
            println!("{}", i - 1);
            return;
        }
    }
}
