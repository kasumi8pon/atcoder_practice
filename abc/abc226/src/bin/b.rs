use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut a = vec![];

    for _i in 0..n {
        input! {
            l: usize,
            line: [usize; l]
        }

        a.push(line);
    }

    a.sort();
    a.dedup();

    println!("{}", a.len());
}
