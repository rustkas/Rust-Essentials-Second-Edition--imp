```rust
fn main() {
    let mut rng = 0..7;

    loop {
        match rng.next() {
            Some(val) => print!("{} - ", val),
            None => break,
        }
    }
}
// 0 - 1 - 2 - 3 - 4 - 5 - 6 -

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=da4301b48811845dcd4a4067e9eae56c&version=stable)
