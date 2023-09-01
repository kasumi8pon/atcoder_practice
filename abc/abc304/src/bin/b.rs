use proconio::input;

fn main() {
    input! {
        n: u64
    }

    let mut digit = 0;
    let mut number = n.clone();

    if number == 0 {
        digit = 1;
    } else {
        while number != 0 {
            digit += 1;
            number /= 10;
        }
    }

    let answer;

    if digit > 3 {
        answer = n - n % (10_u64.pow(digit - 3));
    } else {
        answer = n;
    }

    println!("{}", answer);
}
