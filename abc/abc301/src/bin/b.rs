use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        a: [isize; n]
    }

    let index = n - 1;
    let mut answer = vec![];

    let mut x = 0;

    loop {
        answer.push(a[x]);

        if (x + 1) <= index {
            if (a[x] - a[x + 1]).abs() > 1 {
                if a[x] > a[x + 1] {
                    for number in (a[x + 1] + 1..a[x]).rev() {
                        answer.push(number);
                    }
                } else {
                    for number in a[x] + 1..a[x + 1] {
                        answer.push(number);
                    }
                }
            }
        } else {
            break;
        }

        x += 1;
    }

    println!("{}", answer.iter().join(" "));
}
