```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let magicians = Arc::new(Mutex::new(vec![
        "Morgan".to_string(),
        "Allanon".to_string(),
        "Jafar".to_string(),
    ]));

    for i in 0..3 {
        let magicians = magicians.clone();
        let _ = thread::spawn(move || {
            let mags = magicians.lock();
            match mags {
                Ok(mut mags) => {
                    let mut temp = mags[i].to_string();
                    temp.push_str("ius");
                    mags[i] = temp;
                }
                Err(str) => println!("{}", str),
            }
        })
        .join();
    }
    println!("{:?} - ", *magicians);
    print!("{:?} - ", *magicians.lock().unwrap());
}
// Mutex { data: ["Morganius", "Allanonius", "Jafarius"] } - [Finished in 29.6s]
// ["Morganius", "Allanonius", "Jafarius"]

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=1f77da35f32c0091224ed6f4c3d122a7&version=stable)
