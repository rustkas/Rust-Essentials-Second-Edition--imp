```rust
use PlanetaryMonster::MarsMonster;

#[allow(dead_code)]
enum Compass {
    North,
    South,
    East,
    West,
}

#[allow(non_camel_case_types)]
type species = &'static str;

#[allow(dead_code)]
enum PlanetaryMonster {
    VenusMonster(species, i32),
    MarsMonster(species, i32),
}

#[allow(unused_variables)]
fn main() {
    let direction = Compass::West;
    let martian = PlanetaryMonster::MarsMonster("Chela", 42);
    let martian = MarsMonster("Chela", 42);

    // using enum values:
    // error: binary operation `==` cannot be applied to type `Compass`
    // if direction == Compass::East {
    //  	println!("Go to the east");
    //  }

    match direction {
        Compass::North => println!("Go to the North!"),
        Compass::East => println!("Go to the East!"),
        Compass::South => println!("Go to the South!"),
        Compass::West => println!("Go to the West!"),
    }
}
// Go to the West!

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=4e114c936519bc3de31b74669aa3bba7&version=stable)
