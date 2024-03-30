```rust
use std::mem;

static mut N: i32 = 42;

fn main() {
    let v: &[u8] = unsafe { mem::transmute("Gandalf") };
    println!("{:?}", v);

    // N = 108; // use of mutable static requires unsafe function or block
    // reading or changing a static mutable variable:
    unsafe {
        println!("{:?}", N); // 42
        N = 108;
        println!("{:?}", N); // 108
    }
}
// [71, 97, 110, 100, 97, 108, 102]
// 42
// 108

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=848a6b1dc8b32b5c4abfcbf5dd94ed79&version=stable)
