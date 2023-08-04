use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
        f: usize,
        x: usize
    }

    let t_div = x / (a + c);
    let mut t_rem = x % (a + c);
    if t_rem > a {
        t_rem = a;
    }
    let a_div = x / (d + f);
    let mut a_rem = x % (d + f);
    if a_rem > d {
        a_rem = d;
    }

    let t = t_div * a * b + t_rem * b;
    let a = a_div * d * e + a_rem * e;

    if t > a {
        println!("{}", "Takahashi");
    } else if t < a {
        println!("{}", "Aoki");
    } else {
        println!("{}", "Draw");
    };
}
