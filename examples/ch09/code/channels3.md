```rust
use std::sync::mpsc::channel;
//use std::thread;

fn some_expensive_computation() -> i32 {
    1
}

fn some_other_expensive_computation() {}

fn main() {
    let (tx, rx) = channel();

    let result = some_expensive_computation();
    tx.send(result).ok().expect("Unable to send message");

    some_other_expensive_computation();
    let result = rx.recv();
    println!("{:?}", result); // Ok(1)
}

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=3f95a97d9b323bfadc24782f3265b892&version=stable)
