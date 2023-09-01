use proconio::input;

fn main() {
    input! {
        p: char,
        q: char
    }

    let distance = [0, 3, 4, 8, 9, 14, 23];
    let points =  ['A', 'B', 'C', 'D', 'E', 'F', 'G'];

    let p_index = points.iter().position(|&x| x == p).unwrap();
    let q_index = points.iter().position(|&x| x == q).unwrap();

    println!("{}", ((distance[q_index] - distance[p_index]) as isize).abs());
}
