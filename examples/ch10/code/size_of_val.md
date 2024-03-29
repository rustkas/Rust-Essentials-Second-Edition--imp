```rust
use std::mem;

fn main() {
    let arr = ["Rust", "Go", "Swift"];
    println!("array arr occupies {} bytes", mem::size_of_val(&arr));
    println!("The size of an isize: {} bytes", mem::size_of::<isize>());
}
// array arr occupies 48 bytes
// The size of an isize: 8 bytes
```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=51768a904bb32623f4921fc8e6e51247&version=stable)
