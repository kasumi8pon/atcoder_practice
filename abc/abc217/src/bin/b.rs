use proconio::input;

fn main() {
    input! {
        s1: String,
        s2: String,
        s3: String
    }

    let contest = vec!["ABC", "ARC", "AGC", "AHC"];

    for i in contest {
        if i != &s1 && i != &s2 && i != &s3 {
            println!("{}", i);
        }
    }
}
