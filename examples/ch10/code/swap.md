```rust
use std::mem;

fn main() {
   let mut n = 0;
   let mut m = 1; 
   mem::swap(&mut n, &mut m);
   println!("n: {} m: {}", n, m);
}
// n: 1 m: 0
```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=62cf15dc9566dbabca63163a3c59f9cb&version=stable)
