use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }

    let d = (a.pow(2) as f32 + b.pow(2) as f32).sqrt();

    println!("{} {}", a as f32 / d, b as f32 / d);
}
