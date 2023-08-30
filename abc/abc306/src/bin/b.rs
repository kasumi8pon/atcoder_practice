use proconio::input;

fn main() {
    // input! {
    //     mut numbers: [usize; 64]
    // }

    // numbers.reverse();
    // let joined_number: String = numbers.iter().map(|n| n.to_string()).collect();

    // println!("{}", usize::from_str_radix(&joined_number, 2).unwrap());

    input! {
        numbers: [u64; 64]
    }

    let mut answer = 0;

    for (i, n) in numbers.iter().enumerate() {
        answer += n * 2_u64.pow(i as u32);
    }

    println!("{}", answer);
}
