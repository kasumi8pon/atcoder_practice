use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let digit_1 = (n / 1000) % 10;
    let digit_2 = (n / 100) % 10;
    let digit_3 = (n / 10) % 10;
    let digit_4 = n % 10;

    if digit_1 == digit_2 && digit_2 == digit_3 && digit_3 == digit_4 {
        println!("Weak");
        return;
    }

    if (digit_1 + 1) % 10 == digit_2 && (digit_2 + 1) % 10 == digit_3 && (digit_3 + 1) % 10 == digit_4 {
        println!("Weak");
        return;
    }

    println!("Strong");
}
