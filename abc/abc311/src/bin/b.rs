use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        d: usize,
        s: [Chars; n]
    }

    let mut answers = vec![];

    let mut answer = 0;

    for day in 0..d {
        let all_free = s.iter().all(|schedule| schedule[day] == 'o');

        if all_free {
            answer += 1;
        } else {
            answers.push(answer);
            answer = 0;
        }
    }

    answers.push(answer);

    println!("{}", answers.iter().max().unwrap());
}
