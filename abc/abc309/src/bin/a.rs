use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }

    let mut result = "No";

    if a % 3 == 1 {
        if b == a + 1 {
            result = "Yes";
        }
    } else if a % 3 == 2 {
        if b == a + 1 || b == a - 1 {
            result = "Yes";
        }
    } else if a % 3 == 0 {
        if b == a - 1 {
            result = "Yes";
        }
    }

    println!("{}", result);
}
