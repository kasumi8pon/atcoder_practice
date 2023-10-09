use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n]
    }

    let mut answer1 = 0;
    let mut answer2 = 0;

    for i in 0..n {
        if a[i] == b[i] {
            answer1 += 1;
        }
        for j in 0..n {
            if a[i] == b[j] && i != j {
                answer2 += 1;
            }
        }
    }

    println!("{}", answer1);
    println!("{}", answer2);
}
