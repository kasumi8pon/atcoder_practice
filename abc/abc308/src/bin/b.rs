use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        c: [String; n],
        d: [String; m],
        p: [usize; m + 1]
    }

    let mut result = 0;

    for i in 0..n {
        let index = d.iter().position(|x| *x == c[i]);
        if index.is_some() {
            result += p[index.unwrap() + 1];
        } else {
            result += p[0];
        }
    }

    println!("{}", result);
}
