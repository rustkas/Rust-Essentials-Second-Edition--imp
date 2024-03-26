```rust
#[allow(dead_code)]
#[derive(Clone,Copy)] 
struct Alien<'a> {
    planet: &'a str,
    n_tentacles: u32,
}


#[allow(unused_variables, unused_mut)]
fn main() {
    let mut klaatu = Alien {
        planet: "Venus",
        n_tentacles: 15,
    };

    // a move of the resource:
    // 	let kl2 = klaatu;
    // 	println!("{}", klaatu.planet); // use of moved value 'klaatu.planet'

    // let kl2 = transform(klaatu); // use of moved value: `klaatu`
    // println!("{}", klaatu.planet); // use of moved value 'klaatu.planet'
    // let klaatu = transform(klaatu); // use of moved value: `klaatu`
    // println!("{}", klaatu.planet); // Jupiter

    // a borrowing of the resource:
    let kl2 = &klaatu; // a borrow or reference
    let kl2 = &mut klaatu; // a mutable borrow or reference
    kl2.n_tentacles = 14;
    println!("{} - {}", kl2.planet, kl2.n_tentacles); // Venus - 14

    // ownership is transferred, original owner cannot access or change:
    // error: cannot assign to `klaatu.planet` because it is borrowed
    // klaatu.planet = "Pluto";

    // error: cannot borrow `klaatu.planet` as immutable because `klaatu` is also borrowed as mutable
    // println!("{} - {}", klaatu.planet, klaatu.n_tentacles);
}
#[allow(unused_variables, dead_code)]
fn transform(a: Alien) -> Alien {
    Alien {
        planet: "Jupiter",
        n_tentacles: 0,
    }
}
// Venus - 14
// Jupiter


```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=dcdd18229502ede44a5ee3e4a089dc51&version=stable)
