use proconio::input;

fn main() {
    input! {
        n: usize,
        mut h: [i32; n]
    }

    let max = h.iter().max().unwrap();

    let result = h.iter().position(|x| x == max).unwrap() + 1;

    println!("{}", result);
}
