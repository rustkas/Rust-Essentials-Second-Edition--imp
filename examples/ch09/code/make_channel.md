```rust
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;

fn make_chan() -> Receiver<i32> {
    let (tx, rx) = channel();
    tx.send(7).unwrap();
    rx
}

fn main() {
    let rx = make_chan();
    if let Some(msg) = rx.recv().ok() {
        println!("received message {}", msg);
    };
}
// received message 7

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=f82da8bbca7875ad46803f8c6f08c978&version=stable)
