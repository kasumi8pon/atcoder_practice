use proconio::input;

fn main() {
    input! {
        n: u64,
        t: u64,
        c: [u64; n],
        r: [u64; n]
    }

    let cr: Vec<_> = c.iter().zip(r.iter()).collect();

    let same_colors: Vec<_> = cr.iter().filter(|&&(xc, _xr)| *xc == t).collect();

    let number;
    let answer;

    if same_colors.len() >= 1 {
        number = same_colors.iter().map(|&x| x.1).max().unwrap();
        answer = cr.iter().position(|&(xc, xr)| *xc == t && *xr == *number ).unwrap();
    } else {
        let target_color = cr[0].0;
        number = cr.iter().filter(|&&x| x.0 == target_color).map(|&x| x.1).max().unwrap();
        answer = cr.iter().position(|(xc, xr)| *xc == target_color && *xr == number ).unwrap();
    }

    println!("{}", answer + 1);
}
