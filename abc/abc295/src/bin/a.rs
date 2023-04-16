use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [String; n]
    }

    let target_words = ["and", "not", "that", "the", "you"];

    let mut result = "No";

    for word in w {
        for target_word in target_words.iter() {
            if word == *target_word {
                result = "Yes";
            }
        }
    }

    println!("{}", result);

}
