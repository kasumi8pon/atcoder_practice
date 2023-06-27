use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        c: [usize; n]
    }

    let plus = a + b;

    for i in 0..n {
        if c[i] == plus {
            println!("{}", i + 1);
        }
    }
}
