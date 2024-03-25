```rust
#[allow(unused_must_use,unconditional_panic)]
fn main() {
    // panics:
    let x = 3;
    let y = 0;
    x / y; // thread '<main>' panicked at 'attempted to divide by zero'
}

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=9f22dce4f5e7e2b18e4cb8b84c1229bb&version=stable)
