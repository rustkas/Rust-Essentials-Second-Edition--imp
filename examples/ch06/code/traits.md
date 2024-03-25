```rust
#[allow(dead_code)]
#[derive(Debug)]
struct Alien {
    health: u32,
    damage: u32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Zombie {
    health: u32,
    damage: u32,
}

#[allow(dead_code)]
struct Predator {
    health: u32,
    damage: u32,
}

trait Monster {
    fn new(hlt: u32, dam: u32) -> Self;

    fn attack(&self);
    fn noise(&self) -> &'static str;

    fn attacks_with_sound(&self) {
        println!(
            "The Monster attacks by making an awkward sound {}",
            self.noise()
        );
    }
}

impl Monster for Alien {
    fn new(mut h: u32, d: u32) -> Alien {
        // constraints:
        if h > 100 {
            h = 100;
        }
        Alien {
            health: h,
            damage: d,
        }
    }

    fn attack(&self) {
        println!(
            "I attack! Your health lowers with {} damage points.",
            self.damage
        );
    }

    fn noise(&self) -> &'static str {
        "Shriek!"
    }
}

impl Monster for Zombie {
    fn new(mut h: u32, d: u32) -> Zombie {
        // constraints:
        if h > 100 {
            h = 100;
        }
        Zombie {
            health: h,
            damage: d,
        }
    }

    fn attack(&self) {
        println!(
            "The Zombie bites! Your health lowers with {} damage points.",
            2 * self.damage
        );
    }

    fn noise(&self) -> &'static str {
        "Aaargh!"
    }
}

// Predator still has to implement new and noise methods:
// error[E0046]: not all trait items implemented, missing: `new`, `noise`
// impl Monster for Predator {
// 	fn attack(&self) {
// 		println!("I bite you! Your health lowers with {} damage points.", 3 * self.damage);
// 	}
// }

fn main() {
    let zmb1 = Zombie {
        health: 75,
        damage: 15,
    };
    println!("Oh no, I hear: {}", zmb1.noise());
    zmb1.attack();
    println!("{:?}", zmb1);
    
    let aln1 = Alien::new(75,15);
    println!("Oh no, I hear: {}", aln1.noise());
    aln1.attack();
    println!("{:?}", aln1);
    
}
// Oh no, I hear: Aaargh!
// The Zombie bites! Your health lowers with 30 damage points.
// Zombie { health: 75, damage: 15 }

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=7d814e99e2270dd833de95de0d5b4d12&version=stable)
