```rust
use std::f32::consts;

#[allow(dead_code)]
static MAX_HEALTH: i32 = 100;
static GAME_NAME: &str = "Monster Attack";

fn main() {
	const MYPI: f32 = 3.14;
	println!("{}", MYPI);
    println!("{}", GAME_NAME);
        // use the PI value from the standard library:
	println!("{}", consts::PI);
}

// 3.14
// Monster Attack
// 3.141593
```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=a2a01490973a372ce7dbcd2fa97831fd&version=stable)
