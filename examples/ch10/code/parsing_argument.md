```rust
fn main() {
    if let Some(arg1) = std::env::args().nth(1) {
        if let Ok(x) = arg1.parse::<i32>() {
            println!("Got it: {}", x);
        } else {
            println!("I wasn't given an integer!");
        }
    } else {
        println!("I wasn't given an argument!");
    }
}

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=f5d434d810c19eda8f95afcc109c1a90&version=stable)
