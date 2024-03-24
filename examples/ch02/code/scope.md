```rust
fn main() {
    let outer = 42;
    {
        // start code block
        // This variable inner only exists in this block
        let inner = 3.14;
        println!("block variable: {}", inner);
        let outer = 99; // shadows the first outer variable
        println!("block variable outer: {}", outer);
    } // end of code block
      // println!("out of block: {}", inner);  // error: unresolved name inner - not found in this scope
    println!("outer variable: {}", outer);
}
// block variable: 3.14
// block variable outer: 99
// outer variable: 42

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=63f2cce7feee85ac8bc459a1b1cdb41f&version=stable)
