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

            let mut new_string = strings[i].clone();
            new_string.extend(strings[j].clone());
            let mut reverse_string = new_string.clone();
            reverse_string.reverse();

            if new_string == reverse_string {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
