use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        events: [(usize, usize); q]
    }

    let mut cards = vec![0; n];

    for event in events {
        if event.0 == 1 {
            cards[event.1 - 1] += 1;
        } else if event.0 == 2 {
            cards[event.1 - 1] = 2;
        } else {
            if cards[event.1 - 1] == 2 {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
