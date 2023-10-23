use proconio::input;

fn main() {
    input! {
        n: usize,
        trees: [(usize, usize); n - 1]
    }

    let mut lines = vec![0; n];

    for tree in trees {
        lines[tree.0 - 1] += 1;
        lines[tree.1 - 1] += 1;
    }

    for line in lines {
        if line == n - 1 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
