```rust
/// Calculates the cube `val * val * val`.
///
/// # Examples
///
/// ```
/// let cube = cube(val);
/// ```
pub fn cube(val: u32) -> u32 {
    // implementation goes here
    val * val * val
}


fn main() {
    println!("The cube of 4 is {}", cube(4));
}
// The cube of 4 is 64

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=162cfc34220d64ec07ec0085af46b077&version=stable)
