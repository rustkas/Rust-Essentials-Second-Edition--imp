```rust
#[cfg(target_os = "windows")]
fn on_windows() {
    println!("This machine has Windows as its OS.")
}
// This machine has Windows as its OS.

fn main() {
    on_windows();
}



```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=da162022b97b31f0f601780d18edaa6c&version=stable)
