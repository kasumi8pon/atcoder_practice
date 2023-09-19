use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }

    let mut answer = 0;

    for i in 0..n {
        for j in 0..n {
            if i > j {
                let mut count = true;
                for k in 0..m {
                    if s[i][k] == 'x' && s[j][k] == 'x' {
                        count = false;
                    }
                }
                if count {
                    answer += 1;
                }
            }
        }
    }

    println!("{}", answer);
}
