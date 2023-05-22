use proconio::input;

fn main() {
    input! {
        _n: usize,
        s: String
    }

    let left_index = s.find('|').unwrap();
    let right_index = s.rfind('|').unwrap();
    let star_index = s.find('*').unwrap();

    if left_index < star_index && star_index < right_index {
        println!("in");
    } else {
        println!("out");
    }

}
