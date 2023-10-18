use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars
    }

    let s_indexes = s.iter().filter_map( |&c| Some(c as i32 - 'a' as i32)).collect::<Vec<i32>>();
    let t_indexes = t.iter().filter_map( |&c| Some(c as i32 - 'a' as i32)).collect::<Vec<i32>>();

    let gap = (s_indexes[0] - t_indexes[0] + 26) % 26;

    for i in 1..s_indexes.len() {
        if (s_indexes[i] - t_indexes[i] + 26) % 26 != gap {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
