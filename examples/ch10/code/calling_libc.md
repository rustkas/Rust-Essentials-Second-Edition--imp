```rust
#![feature(libc)] // only for use on the nightly release
extern crate libc;
use libc::puts;
use std::ffi::CString;

fn main() {
	let sentence = "Merlin is the greatest magician!";
    let to_print = CString::new(sentence).unwrap();
    unsafe {
        puts(to_print.as_ptr());
    }
}
// Merlin is the greatest magician!
```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=03114b15d4c2adc2506451eaf9306d13&version=stable)
