use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars
    }

    let s_size = s.len();
    let t_size = t.len();

    for i in 0..s_size {
        for j in 0..s_size {
            if i > j {
                continue;
            } else if j - i + 1 != t_size {
                continue;
            } else {
                let mut index = 0;
                let mut same = true;
                for k in i..=j {
                    if s[k] != t[index] {
                        same = false;
                        break;
                    }
                    index += 1;
                }
                if same {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No")
}
