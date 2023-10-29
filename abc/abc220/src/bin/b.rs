use proconio::input;

fn main() {
    input! {
        k: u32,
        a: String,
        b: String,
    }

    let x = usize::from_str_radix(&a, k).unwrap();
    let y = usize::from_str_radix(&b, k).unwrap();

    println!("{}", x * y);
}
