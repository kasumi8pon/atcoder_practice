use proconio::input;

fn main() {
    input! {
        t: usize
    }

    for _ in 0..t {
        input! {
            n: usize,
            a: [usize; n]
        }

        let answer = a.iter().filter(|&x| x % 2 != 0).count();

        println!("{}", answer);
    }
}
