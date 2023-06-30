use proconio::input;

fn main() {
    input! {
        n: usize,
        persons: [(String, usize); n]
    }

    let min_index = (0..n).min_by_key(|&i| persons[i].1).unwrap();

    for i in 0..n {
        let index = (min_index + i) % n;
        println!("{}", persons[index].0)
    }
}
