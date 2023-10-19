use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let size = s.len();

    if s[0] == 'o' {
        for i in 0..size {
            if i % 3 == 0 {
                if s[i] != 'o' {
                    println!("No");
                    return;
                }
            } else {
                if s[i] != 'x' {
                    println!("No");
                    return;
                }
            }
        }
    } else if s[0] == 'x' {
        if size > 1 {
            if s[1] == 'o' {
                for i in 0..size {
                    if i % 3 == 1 {
                        if s[i] != 'o' {
                            println!("No");
                            return;
                        }
                    } else {
                        if s[i] != 'x' {
                            println!("No");
                            return;
                        }
                    }
                }
            } else {
                for i in 0..size {
                    if i % 3 == 2 {
                        if s[i] != 'o' {
                            println!("No");
                            return;
                        }
                    } else {
                        if s[i] != 'x' {
                            println!("No");
                            return;
                        }
                    }
                }
            }
        }
    }

    println!("Yes");
}
