use proconio::input;

fn main() {
    input! {
        n: usize,
        names: [(String, String); n]
    }

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            if names[i].0 == names[j].0 && names[i].1 == names[j].1 {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
