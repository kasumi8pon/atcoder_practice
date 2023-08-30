use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        strings: [Chars; n]
    }

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            let new_string: Vec<_> = strings[i].clone().into_iter().chain(strings[j].clone().into_iter()).collect();
            let reverse_string: Vec<_> = new_string.clone().into_iter().rev().collect();

            if new_string == reverse_string {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
