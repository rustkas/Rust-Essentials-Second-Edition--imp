```rust
use std::sync::mpsc;
use std::thread;

trait Message: Send {
    fn print(&self);
}

struct Msg1 {
    value: i32,
}

impl Message for Msg1 {
    fn print(&self) {
        println!("value: {:?}", self.value);
    }
}

fn main() {
    let (tx, rx) = mpsc::channel::<Box<dyn Message>>();

    let handle = thread::spawn(move || {
        let msg = rx.recv().unwrap();
        msg.print();
    });

    let msg = Box::new(Msg1 { value: 1 });
    tx.send(msg).unwrap();

    handle.join().ok();
}
// value: 1

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=62184e525cd764de7e7db00a20a789c7&version=stable)
