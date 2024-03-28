```rust
use std::thread;

fn main() {
	let result = thread::spawn(move || {
    	panic!("I have fallen into an unrecoverable trap!");
	}).join();

	if result.is_err() {
		println!("This child has panicked");
	}
}
// thread '<unnamed>' panicked at 'I have fallen into an unrecoverable trap!'
// This child has panicked

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=519f857047747e9e2ad62fad8c30eca5&version=stable)
