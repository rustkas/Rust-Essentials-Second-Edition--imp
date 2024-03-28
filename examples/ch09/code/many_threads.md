```rust
use std::thread;
use std::time;

static NTHREADS: i32 = 1000;

fn main() {
    println!("************************** Before the start of the threads");
    for i in 0..NTHREADS {
        thread::spawn(move || println!("this is thread number {}", i));
    }
    // thread::sleep(time::Duration::from_millis(50));
    println!("************************** All threads finished!");
}

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=abba0ff664f9c210911c801bb3229017&version=stable)
