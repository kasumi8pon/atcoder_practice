use proconio::input;

fn main() {
    input! {
        x1: isize,
        y1: isize,
        x2: isize,
        y2: isize,
        x3: isize,
        y3: isize,
    }

    let answer_x;

    if x1 == x2 {
        answer_x = x3;
    } else if x1 == x3 {
        answer_x = x2;
    } else {
        answer_x = x1;
    }

    let answer_y;

    if y1 == y2 {
        answer_y = y3;
    } else if y1 == y3 {
        answer_y = y2;
    } else {
        answer_y = y1;
    }

    println!("{} {}", answer_x, answer_y);
}
