use proconio::input;

fn main() {
    input! {
        s: usize,
        t: usize,
        x: usize
    }

    if s < t{
        if s <= x && x < t {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        if x < t || s <= x {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
