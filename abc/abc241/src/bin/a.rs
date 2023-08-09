use proconio::input;

fn main() {
    input! {
        a: [usize; 10]
    }

    let first_index = 0;
    let second_index = a[first_index];
    let third_index = a[second_index];

    println!("{}", a[third_index]);
}
