use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut called = vec![false; n];

    for i in 0..n {
        if called[i] == false {
            called[a[i] - 1] = true;
        }
    }

    let count = called.iter().filter(|&&x| x == false).count();

    println!("{}", count);
    for i in 0..n {
        if called[i] == false {
            print!("{} ", i + 1);
        }
    }
}
