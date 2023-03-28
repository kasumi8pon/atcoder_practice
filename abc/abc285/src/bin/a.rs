use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let result;

    if b / 2 == a {
        result = "Yes";
    } else {
        result = "No";
    }

    println!("{}", result)
}
