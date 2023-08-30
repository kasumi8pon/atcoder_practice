use proconio::input;

fn main() {
    input! {
        mut numbers: [usize; 64]
    }

    numbers.reverse();
    let joined_number: String = numbers.iter().map(|n| n.to_string()).collect();

    println!("{}", usize::from_str_radix(&joined_number, 2).unwrap());
}
