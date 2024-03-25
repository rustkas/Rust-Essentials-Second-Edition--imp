```rust
struct Block {
    number: Box<i32>,
}

impl Clone for Block {
    fn clone(&self) -> Self {
        Block { number: self.number.clone() }
    }
}

fn print_block(block: Block) {
    println!("{:p}: {:?}", block.number, block.number);
}

fn main() {
    let block = Block { number: Box::new(1) };
    println!("{:p}: {:?}", block.number, block.number);
    print_block(block.clone());
}
// 0x20c5ca23b00: 1
// 0x20c5ca2cbe0: 1
```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=359aab8ff677fa29d84cf4b5a2bc1382&version=stable)
