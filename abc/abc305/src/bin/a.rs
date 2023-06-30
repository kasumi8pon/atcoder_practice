use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let nmod = n % 5;

    if nmod == 0 {
        println!("{}", n);
    } else if nmod == 1 || nmod == 2 {
        println!("{}", n - nmod);
    } else {
        println!("{}", n + (5 - nmod));
    }
}
