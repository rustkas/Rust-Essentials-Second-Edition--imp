```rust
#[allow(non_upper_case_globals)]
static mut globvar: i32 = 42;

fn main() {
    // error: use of mutable static requires unsafe function or block [E0133]
    unsafe {
        // because it is dangerous to change a global variable!
        globvar = 0;
        println!("My variable global constant: {}", globvar);
    }
}
// My variable global constant: 0

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=18518384df734c25be4064d32baf79ec&version=stable)
