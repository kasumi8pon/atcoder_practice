use proconio::input;

fn main() {
    input! {
        s: String
    }

    let strings = vec!["ACE", "BDF", "CEG", "DFA", "EGB", "FAC", "GBD"];

    let position = strings.iter().position(|&string| string == s);

    match position {
        Some(_) => println!("Yes"),
        None => println!("No"),
    }
}
