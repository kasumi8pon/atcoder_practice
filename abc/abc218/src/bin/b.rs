use proconio::input;

fn main() {
    input! {
        p: [u8; 26]
    }

    for i in p {
        print!("{}", (b'a' + i - 1 as u8) as char);
    }
}
