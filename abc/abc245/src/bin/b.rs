use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort();
    a.dedup();

    let mut counter = 0;

    for i in 0..=2000 {
        if counter == a.len() {
            println!("{}", i);
            return;
        }

        if i < a[counter] {
            println!("{}", i);
            return;
        } else {
            counter += 1;
        }
    }
}
