```rust
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
struct Alien<'a> {
    name: &'a str,
    no_tentacles: u8,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Tentacle<'a> {
    poison: u8,
    owner: Alien<'a>,
}

fn main() {
    let dhark = Alien {
        name: "Dharkalen",
        no_tentacles: 7,
    };
    println!("{:?}", dhark);
    // defining dhark's tentacles:
    for i in 0u8..dhark.no_tentacles {
        let tentacle = Tentacle {
            poison: i * 3,
            owner: dhark,
        };
        if i > 0 {
            break;
        }
        println!("{:?}", tentacle);
    }
}

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=e3ac4d834341e69c17fff409e02f824a&version=stable)
