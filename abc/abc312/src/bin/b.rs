use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n]
    }

    let mut answers = vec![];

    for i in 0..=(n-9) {
        for j in 0..=(m-9) {
            if s[i][j] == '#' && s[i][j+1] == '#' && s[i][j+2] == '#' && s[i][j+3] == '.'
            && s[i+1][j] == '#' && s[i+1][j+1] == '#' && s[i+1][j+2] == '#' && s[i+1][j+3] == '.'
            && s[i+2][j] == '#' && s[i+2][j+1] == '#' && s[i+2][j+2] == '#' && s[i+2][j+3] == '.'
            && s[i+3][j] == '.' && s[i+3][j+1] == '.' && s[i+3][j+2] == '.' && s[i+3][j+3] == '.'
            && s[i+5][j+5] == '.' && s[i+5][j+6] == '.' && s[i+5][j+7] == '.' && s[i+5][j+8] == '.'
            && s[i+6][j+5] == '.' && s[i+6][j+6] == '#' && s[i+6][j+7] == '#' && s[i+6][j+8] == '#'
            && s[i+7][j+5] == '.' && s[i+7][j+6] == '#' && s[i+7][j+7] == '#' && s[i+7][j+8] == '#'
            && s[i+8][j+5] == '.' && s[i+8][j+6] == '#' && s[i+8][j+7] == '#' && s[i+8][j+8] == '#' {
                answers.push((i+1, j+1));
            }
        }
    }

    answers.iter().for_each(|(i, j)| {
        println!("{} {}", i, j);
    });
}
