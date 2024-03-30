```rust
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = BufReader::new(File::open("numbers.txt").unwrap());

    // .lines() eats EOF error, so this will only panic if something went completely wrong.
    let pairs: Vec<_> = file
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let line = line.trim();
            let mut words = line.split(" ");
            let left = words.next().expect("Unexpected empty line!");
            let right = words.next().expect("Expected number!");

            (
                left.parse::<u64>()
                    .ok()
                    .expect("Expected integer in first column!"),
                right
                    .parse::<f64>()
                    .ok()
                    .expect("Expected float in second column!"),
            )
        })
        .collect();

    println!("{:?}", pairs);
}
// [(120, 345.56), (125, 341.56)]

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=8a5b467db41e5f4926bb964b372d3b1d&version=stable)
