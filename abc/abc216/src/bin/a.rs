use proconio::input;

fn main() {
    input! {
        s: String
    }

    let xy: Vec<&str> = s.split(".").collect();
    let x = xy[0].parse::<usize>().unwrap();
    let y = xy[1].parse::<usize>().unwrap();

    if y <= 2 {
        println!("{}-", x);
    } else if 3 <= y && y <= 6 {
        println!("{}", x);
    } else {
        println!("{}+", x);
    }
}
