use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n]
    }

    let mut know = vec![false; n];

    let mut target = x - 1;

    loop {
        if know[target] {
            break;
        } else {
            know[target] = true;
            target = a[target] - 1;
        }
    }

    println!("{}", know.iter().filter(|&&x| x).count());
}
