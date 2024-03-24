```rust
fn main() {
    foo();
}

fn foo() {
    fn bar() {
        println!("bar");
    }
    bar();
}
// bar

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=333f01b84143fc4334a1a0fce7a20214&version=stable)
