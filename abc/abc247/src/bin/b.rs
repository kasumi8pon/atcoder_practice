use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut names = vec![];

    for _i in 0..n {
        input! {
            s: String,
            t: String
        }

        names.push((s, t));
    }

    for name in &names {
        if names.iter().filter(|&n| n.0 == name.0 || n.1 == name.0).count() > 1 && names.iter().filter(|&n| n.0 == name.1 || n.1 == name.1).count() > 1
        {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
