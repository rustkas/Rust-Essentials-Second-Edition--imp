```rust
#[allow(unused_variables, unused_assignments)]
fn main() {
    let points = 10i32;
    let mut saved_points: u32 = 0;
    // saved_points = points; // error
    // error: mismatched types: expected u32, found i32
    saved_points = points as u32;

    let f2 = 3.14;
    // truncation occurs here:
    saved_points = f2 as u32;
    println! ("{}", saved_points); // 3  

    let mag = "Gandalf";
    // saved_points = mag as u32; // error: non-scalar cast: `&str` as `u32`
}
```

[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=4037c8fa92d5e40e6017e43ee3a941cc&version=stable)
