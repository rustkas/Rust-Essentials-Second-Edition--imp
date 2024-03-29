```rust
fn main() {
    let items = [1u32, 2, 3, 4];
    let ptr = &items[1] as *const u32;
    println!("{}", unsafe { *ptr });
    println!("{}", unsafe { *ptr.offset(-1) });
    println!("{}", unsafe { *ptr.offset(1) });
    // error: binary operation `+` cannot be applied to type `*const u32` [E0369]
    // println!("{}", unsafe { ptr + 2});
}
// 2
// 1
// 3
```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=2d45db94732eae06cc5fd091203dab22&version=stable)
