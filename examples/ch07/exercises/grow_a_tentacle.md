```rust
#[allow(dead_code)]
struct Alien {
    planet: String,
    no_tentacles: u32,
}

fn grow_a_tentacle(al: &mut Alien) {
    al.no_tentacles += 1;
} // al goes out of scope

fn main() {
    let mut klaatu = Alien {
        planet: "Venus".to_string(),
        no_tentacles: 15,
    };
    println!("Klaatu first has  {} tentacles", klaatu.no_tentacles); // 15
    grow_a_tentacle(&mut klaatu);
    println!("Klaatu has now {} tentacles", klaatu.no_tentacles); // 16
}



// Klaatu has first 15 tentacles
// Klaatu has now 16 tentacles

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=fdfeafd87d486492a826b4b50bf72fd8&version=stable)
