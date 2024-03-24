```rust
struct Monster {
    health: i32,
    damage: i32,
}

fn main() {
    let m = Monster {
        health: 10,
        damage: 20,
    };

    println!("{}", m.health);
    println!("{}", m.damage);
}
// 10
// 20

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=d13c1b8dc58e4ca91b7466bb5e64d208&version=stable)
