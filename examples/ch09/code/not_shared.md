```rust
use std::thread;
use std::time;

#[allow(unused_variables,unused_assignments)]
fn main() {
    let mut health = 12;
    for i in 2..5 {
        thread::spawn(move || {
            health *= i;
        });
    }
    thread::sleep(time::Duration::from_secs(2));
    println!("{}", health); // 12
}
```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=508dc6aacc2c8b40cc0ebc808fb5261a&version=stable)
