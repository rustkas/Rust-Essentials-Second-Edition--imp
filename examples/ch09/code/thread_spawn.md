```rust
use std::thread;
use std::time;

#[allow(unused_variables, unused_must_use)]
fn main() {
    let handle = thread::spawn(move || {
        println!("Hello from the goblin in the spawned thread!");
    });

    thread::sleep(time::Duration::from_millis(50));

    // do other work in the meantime

    // wait on child thread to end
    let output = handle.join().unwrap(); // ()
    println!("{:?}", output);

    // wait on child thread, if no other work has to be done:
    thread::spawn(move || {
        println!("Hello again from the goblin in the spawned thread!");
        // other work done in child thread 
    }).join();

    let child = thread::Builder::new().stack_size(32 * 1024 * 1024).spawn(move || {
    // code to be executed in thread
    }).unwrap();

}
// Hello from the goblin in the spawned thread!
// ()
// Hello again from the goblin in the spawned thread!
```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=61484c5f9ecfe04bf57142606f643be9&version=stable)
