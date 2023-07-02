use proconio::input;

fn main() {
    input! {
        s: [usize; 8]
    }

    let mut result = "Yes";

    for n in 0..8 {
        if s[n] % 25 == 0 && s[n] >= 100 && s[n] <= 675 {
            if n == 0 || s[n - 1] <= s[n] {
                continue;
            } else {
                result = "No";
                break;
            }
        } else {
            result = "No";
            break;
        }
    }

    println!("{}", result);
}
