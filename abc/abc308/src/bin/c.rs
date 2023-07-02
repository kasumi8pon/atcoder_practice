use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n]
    }

    let mut pairs: Vec<((i64, i64), usize)> = ab.iter().enumerate().map(|(i, &(a, b))| ((a, b), i)).collect();

    pairs.sort_by(|(x, i), (y, j)| { (y.0 * x.1).cmp(&(x.0 * y.1)).then_with(|| i.cmp(j)) });

    let result: String = pairs.iter().map(|&(_, i)| (i + 1).to_string()).collect::<Vec<String>>().join(" ");

    println!("{}", result);
}
