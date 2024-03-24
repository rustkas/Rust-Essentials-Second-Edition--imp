```rust
fn main() {
    let x = (59, false);

    match x {
        // (n, true) if (n >= 20 && n <= 26) => println!("true and between 20 and 26"),
        (20..=26, true) => println!("true and between 20 and 26"),
        (n, true) if !(n >= 20 && n <= 26) => println!("true and not between 20 and 26"),
        (40..=49, _) => println!("between 40 and 49"),
        (_, _) => println!("Everything else"),
    }
}
```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=bc01c49871cdb3babcc9778638e4d132&version=stable)
