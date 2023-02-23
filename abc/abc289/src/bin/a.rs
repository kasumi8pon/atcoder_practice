use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let t = s.chars().map(|c| match c {
        '0' => '1',
        '1' => '0',
        _ => unreachable!()
    }).collect::<String>();

    println!("{}", t);
}
