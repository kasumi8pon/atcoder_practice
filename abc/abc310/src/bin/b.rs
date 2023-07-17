use proconio::input;

fn main() {
    input! {
        n: usize,
        _m: usize
    }

    let mut products = vec![];

    for _ in 0..n {
        input! {
            p: usize,
            c: usize,
            f: [usize; c]
        }

        products.push((p, f));
    }

    let mut result = "No";

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            if products[i].0 >= products[j].0 {
                if products[i].1.iter().all(|&x| products[j].1.contains(&x)) {
                    if products[i].0 > products[j].0 {
                        result = "Yes";
                    } else if products[i].1.len() < products[j].1.len() {
                        result = "Yes";
                    } else {
                        continue;
                    }
                }
            }
        }
    }

    println!("{}", result)
}
