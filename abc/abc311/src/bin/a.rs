use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        s: Chars
    }

    let mut exist_a = false;
    let mut exist_b = false;
    let mut exist_c = false;
    let mut answer = 0;

    for (i, &c) in s.iter().enumerate() {
        match c {
            'A' => exist_a = true,
            'B' => exist_b = true,
            'C' => exist_c = true,
            _ => ()
        }

        if exist_a && exist_b && exist_c {
            answer = i + 1;
            break;
        }
    };

    println!("{}", answer);
}
