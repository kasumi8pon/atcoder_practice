use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        mut a: [usize; n]
    }

    let mut answers = vec![false; w];

    for i in 0..n {
        let sum = a[i];

        if sum <= w {
            answers[sum - 1] = true;
        }

        for j in 0..n {
            if j == i {
                continue;
            }

            let sum = sum + a[j];

            if sum <= w {
                answers[sum - 1] = true;
            }

            for k in 0..n {
                if k == j || k == i {
                    continue;
                }
                let sum = sum + a[k];

                if sum <= w {
                    answers[sum - 1] = true;
                }
            }
        }
    }

    println!("{}", answers.iter().filter(|&&x| x).count());
}
