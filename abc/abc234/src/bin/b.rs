use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n]
    }

    let mut distances = vec![];

    for i in 0..n {
        for j in 0..n {
            if i != j {
                let x1 = xy[i].0;
                let y1 = xy[i].1;
                let x2 = xy[j].0;
                let y2 = xy[j].1;

                let distance = (x1 - x2).pow(2) + (y1 - y2).pow(2);

                distances.push(distance);
            }
        }
    }

    let max = distances.iter().max().unwrap();

    println!("{}", (*max as f64).sqrt());
}
