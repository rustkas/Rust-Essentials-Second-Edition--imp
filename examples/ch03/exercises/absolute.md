```rust
fn main() {
    println!("{}", abs(-124));
    assert_eq!(5, abs(-5));
}

// error version:
// fn abs(x: i32) -> u32 {
//    if x > 0 { x }
//    else { -x }
// }
// lines 6 and 7:
// error[E0308]: mismatched types: expected u32, found i32

// corrected version:
fn abs(x: i32) -> u32 {
    if x > 0  { 
   	    x as u32 
    } else { 
   	    -x as u32
    }
}
```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=d224c1a7becd1df5555df3340e2fd53d&version=stable)
