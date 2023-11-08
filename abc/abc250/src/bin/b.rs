use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize
    }

    for i in 0..n * a {
        for j in 0..n * b {
            if (i / a) % 2 == 0 {
                if (j / b) % 2 == 0 {
                    print!(".");
                } else {
                    print!("#");
                }
            } else {
                if (j / b) % 2 == 0 {
                    print!("#");
                } else {
                    print!(".");
                }
            }
        }
        println!();
    }
}
