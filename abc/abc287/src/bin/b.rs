use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m]
    }

    let answer = s.iter().filter(|&x| t.iter().any(|y| x.ends_with(y))).count();

    println!("{}", answer);
}
