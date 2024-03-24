```rust
fn main() {
    // exhaustive match with _:
    let magician = "Sauron";
    match magician {
        "Gandalf" => println!("A good magician!"),
        _ => println!("No magician turned up!"),
        //	"Sauron"  => println!("A magician turned bad!") // error: unreachable pattern [E0001]
    }
}

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=4bb5c6cb8d03ec5cea1c7b1f7b462346&version=stable)
