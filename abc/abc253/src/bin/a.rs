use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    }

    let max = a.max(c);
    let min = a.min(c);

    if b >= min && b <= max {
        println!("Yes");
    } else {
        println!("No");
    }
}
