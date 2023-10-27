use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars
    }

    let size = s.len();
    let mut unmatched = vec![];

    for i in 0..size {
        if s[i] != t[i] {
            unmatched.push(i);
        }
    }

    if unmatched.len() == 0 {
        println!("Yes");
    } else if unmatched.len() == 2 {
        if unmatched[1] - unmatched[0] == 1 {
            if s[unmatched[0]] == t[unmatched[1]] && s[unmatched[1]] == t[unmatched[0]] {
                println!("Yes");
            } else {
                println!("No");
            }
        } else {
            println!("No");
        }
    } else {
        println!("No");
    }
}
