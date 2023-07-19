use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut fk = vec![1];

    for i in 1..=n {
        let fi = i * fk[i - 1];
        fk.push(fi);
    }

    println!("{}", fk[n]);
}
