use proconio::input;

fn main() {
    input! {
        t: usize
    }

    fn f(x: usize) -> usize {
        x.pow(2) + 2 * x + 3
    }

    let answer = f(f(f(t) + t) + f(f(t)));

    println!("{}", answer);
}
