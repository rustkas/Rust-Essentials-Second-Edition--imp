```rust
fn fib(x: i64) -> i64 {
    if x == 0 || x == 1 {
        return x;
    }
    fib(x - 1) + fib(x - 2)
}

fn main() {
    let ans = fib(10);
    println!("{}", ans);
}
// 55

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=454fa76643d701a10d8f6e20af02f5e2&version=stable)
