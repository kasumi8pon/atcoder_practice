use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
        b: [usize; k]
    }

    let max = a.iter().max().unwrap();

    for i in 0..n {
        if a[i] == *max {
            let find = b.iter().find(|&&x| x == i + 1);
            if find.is_some() {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
