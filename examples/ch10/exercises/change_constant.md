```rust
static mut MAXHEALTH: i32 = 100;
static mut GAMENAME: &str = "Monster Attack";

fn main() {
    unsafe {
        MAXHEALTH = 99;
        GAMENAME = "Monster win";
    }
}


```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=8a1a0f3db827d4bc2ca079754d3a29b2&version=stable)
