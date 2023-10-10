use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
        b: [usize; m]
    }

    for size in b.iter() {
        let position = a.iter().position(|&x| x == *size);
        match position {
            Some(n) => { a[n] = 0; },
            None => { println!("No"); return; }
        }
    }

    println!("Yes");
}
