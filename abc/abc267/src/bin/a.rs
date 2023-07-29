use proconio::input;

fn main() {
    input! {
        s: String
    }

    let days = vec!["Friday", "Thursday", "Wednesday", "Tuesday", "Monday"];

    let position = days.iter().position(|&day| day == s).unwrap();

    println!("{}", position + 1);
}
