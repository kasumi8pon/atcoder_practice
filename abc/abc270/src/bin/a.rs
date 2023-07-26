use proconio::input;

fn main() {
    input! {
        mut a: usize,
        mut b: usize
    }

    // let scores = vec![4, 2, 1];
    // let mut answer = vec![];

    // scores.iter().for_each(|&score| {
    //     if a >= score {
    //         a -= score;
    //         answer.push(score);
    //     }

    //     if b >= score {
    //         b -= score;
    //         answer.push(score);
    //     }
    // });

    // answer.dedup();
    // let total: usize = answer.iter().sum();

    // println!("{}", total);

    println!("{}", a | b);
}
