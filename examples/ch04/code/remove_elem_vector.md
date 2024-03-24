```rust
fn main() {
    let mut vec1: Vec<i32> = (0..10).collect();
    vec1.retain(|&x| !is_odd(x));
    println!("{:?}", vec1)
}

fn is_odd(n: i32) -> bool {
    n % 2 != 0
}
// [0, 2, 4, 6, 8]

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=d210fedac63c337409a6de0609e7926a&version=stable)
