use proconio::input;

fn main() {
    input! {
        k: usize
    }

    let hour = 21 + (k / 60);
    let minute = k % 60;

    println!("{}:{:02}", hour, minute);
}
