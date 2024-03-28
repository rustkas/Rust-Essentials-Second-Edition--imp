```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut health = 12;
    println!("health before: {:?}", health);
    let data = Arc::new(Mutex::new(health));
    for i in 2..5 {
        let mutex = data.clone();
        let _ = thread::spawn(move || {
            let health = mutex.lock();
            match health {
                // health is multiplied by i:
                Ok(mut health) => *health *= i,
                Err(str) => println!("{}", str),
            }
        })
        .join();
    }
    health = *data.lock().unwrap();
    println!("health after: {:?}", health);
}
// health before: 12
// health after: 288
// because: 288 = 12 * 2 * 3 * 4

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=c79e9b55a525587fea35d5c1625dfd39&version=stable)
