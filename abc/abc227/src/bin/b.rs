use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [usize; n]
    }

    let mut candidates = vec![];

    for a in 1..=1000 {
        for b in 1..=1000 {
            candidates.push(4 * a * b + 3 * a + 3 * b);
        }
    }

    candidates.sort();
    candidates.dedup();

    let mut answer = 0;

    for t in s {
        if !candidates.contains(&t) {
            answer += 1;
        }
    }

    println!("{}", answer);
}
