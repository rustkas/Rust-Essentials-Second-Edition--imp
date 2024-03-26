```rust
struct Alien {
    planet: String,
    no_tentacles: u32,
}

#[allow(unused_mut, unused_variables)]
fn main() {
    let mut klaatu = Alien {
        planet: "Venus".to_string(),
        no_tentacles: 15,
    };

    // Question 1)
    let mut kl2 = &mut klaatu;

    kl2.no_tentacles = 14;
    println!("{} - {}", kl2.planet, kl2.no_tentacles); // Venus - 14

    klaatu.planet = "Pluto".to_string();
    println!("{} - {}", klaatu.planet, klaatu.no_tentacles); // Venus - 14

    // Question 2) - with the following statement:
    let klaatuc = klaatu;
    // let kl2 = &klaatu;
    let kl2 = &klaatuc;
 
    // mutability can be changed when ownership is transferred:
    let mut im = Box::new(7u32);
    *im = 4;
    // Hand over the box, changing mutability
    let mut muta = im;
    println!("muta contains {}", muta);
    // println!("im contains {}", im); // error: use of moved value `im`
    println!("im contains {}", muta);
    // Modify the contents of the box
    *muta = 42;
    println!("muta now contains {}", muta);
}
// Venus - 15
// muta contains 7
// muta now contains 42

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=b9c2772469a458bde9a63ae95805caeb&version=stable)
