use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; 4 * n - 1]
    }

    let mut counter = vec![0; n];

    for number in a {
        counter[number - 1] += 1;
    }

    let answer = counter.iter().position(|&x| x == 3).unwrap() + 1;

    println!("{}", answer);
}
