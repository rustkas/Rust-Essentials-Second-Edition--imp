```rust
fn main() {
    let dec = 3.2f32;
    // should be printed out as +003.20

    println!("{}", dec); // 3.2
    println!("{:+007.2}", dec); // +003.20

    // explanation:
    // +00 = literal text
    // 7 = total character width of output
    // .2 = 2 digits after decimal point

    println!("{:+07.2}", dec);
}

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=5e8263bcc04c730964d9a7ef47ced514&version=stable)
