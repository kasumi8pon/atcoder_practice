use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut p = 0;
    let mut bases = vec![false; 4];

    for i in 0..n {
        bases[0] = true;

        for j in vec![3, 2, 1, 0] {
            if bases[j] == true {
                if a[i] + j >= 4 {
                    p += 1;
                } else {
                    bases[a[i] + j] = true;
                }
                bases[j] = false;
            }
        }
    }

    println!("{}", p);
}
