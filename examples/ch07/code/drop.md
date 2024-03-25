```rust
#[allow(dead_code)]
struct Block {
    number: i32,
}

impl Drop for Block {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}

#[allow(unused_variables)]
fn print_block(block: Block) {
    println!("In function print_block");
}

fn main() {
    let block = Block { number: 1 };
    // move of value block:
    print_block(block);
    println!("Back in main!");
}
// In function print_block
// Dropping!
// Back in main!

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=ac4f5288d847646b5f45fe76c4f064ae&version=stable)
