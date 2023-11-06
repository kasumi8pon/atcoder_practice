use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    if n == 1 {
        if a[0] < 180 {
            println!("{}", 360 - a[0]);
        } else {
            println!("{}", a[0]);
        }
    } else {
        let mut cuts = vec![0];
        let mut current_angle = 0;

        for i in a {
            let angle = (current_angle + i) % 360;
            current_angle = angle;

            cuts.push(angle);
        }

        cuts.sort();

        let mut angles = vec![];

        for i in 1..=n {
            angles.push(cuts[i] - cuts[i - 1]);
        }

        println!("{}", angles.iter().max().unwrap());
    }
}
