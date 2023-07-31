use proconio::input;

fn main() {
    input! {
        mut numbers: [usize; 5]
    }

    numbers.sort();
    let mut unique_numbers = numbers.clone();
    unique_numbers.dedup();

    if unique_numbers.len() == 2 {
        let first_number = unique_numbers[0];
        let last_number = unique_numbers[1];
        if numbers.iter().filter(|x| *x == &first_number).count() == 2 && numbers.iter().filter(|x| *x == &last_number).count() == 3
            || numbers.iter().filter(|x| *x == &first_number).count() == 3 && numbers.iter().filter(|x| *x == &last_number).count() == 2 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
