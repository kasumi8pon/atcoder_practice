use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String;n]
    }

    let mut for_count = 0;

    for i in 0..n {
        if s[i] == "For" {
            for_count += 1;
        }
    }

    if for_count > n / 2 {
        print!("Yes");
    } else {
        print!("No");
    }
}
