use proconio::input;

fn main() {
    input! {
        mut numbers: [usize; 5]
    }

    numbers.sort();
    numbers.dedup();

    println!("{}", numbers.len());
}
