use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: usize
    }

    let rem = k % n;

    let mut answer = (a + rem - 1) % n;

    if answer == 0 {
        answer = n;
    }

    println!("{}", answer);
}
