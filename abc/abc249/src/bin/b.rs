use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars
    }

    let all_uppercase = s.iter().all(|&c| c.is_uppercase());
    let all_lowercase = s.iter().all(|&c| c.is_lowercase());

    let origin_size = s.len();
    s.sort();
    s.dedup();
    let dedup_size = s.len();

    let unique = origin_size == dedup_size;

    if !all_uppercase && !all_lowercase && unique {
        println!("Yes");
    } else {
        println!("No");
    }
}
