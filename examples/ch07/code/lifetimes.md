```rust
#[allow(dead_code)]
struct Magician {
    name: &'static str,
    power: u32,
}

// this code does not compile:
// error: missing lifetime specifier [E0106]
// struct MagicNumbers {
// 	magn1: &u32,
// 	magn2: &u32
// }

// this code is ok, both the struct and the fields have lifetime 'a:
#[allow(dead_code)]
struct MagicNumbers<'a> {
    magn1: &'a u32,
    magn2: &'a u32,
}

// #[derive(Debug)]
// #[derive(Copy, Clone)]
#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
struct MagicNumber {
    value: u64,
}

// impl Copy for MagicNumber {}
// impl Clone for MagicNumber {
// 	  fn clone(&self) -> MagicNumber {
//    	  *self
// 	  }
// }

#[allow(dead_code)]
struct MagicNumber2 {
    value: u64,
}


fn life(m: u32) -> u32 {
    let o = m;
    o
}

#[allow(unused_variables, dead_code)]
fn transform<'a>(s: &'a str) { /* ... */
}

#[allow(unused_variables, dead_code)]
fn transform_without_lifetime(s: &str) { /* ... */
}


#[allow(unused_variables)]
fn main() {
    // lifetimes restricted to a function:
    let n = 42u32;
    // copy behaviour:
    // no move, only a copy of the value from n to n2:
    let n2 = n;
    println!("The value of n2 is {}, the same as n", n2);

    let p = life(n);
    println!("p is: {}", p); // p is: 42
                             // println!("{}", m);  // error: unresolved name `m`.
                             // println!("{}", o);  // error: unresolved name `o`.

    // lifetime restricted to a code block:
    {
        let phi = 1.618;
    }
    // error: unresolved name `phi`.
    // println!("The value of phi is {}", phi);

    // let m = return_magician();
    // println!("{} has {}", m.name, m.power);

    // copies because MagicNumber implements the Copy or Clone trait
    let mag = MagicNumber { value: 42 };
    let mag2 = mag;
    let mag3 = mag;

    // mag, mag2 and mag3 are 3 different objects: their addresses are different:
    println!("address mag:  {:p}", &mag); // address is 0x6ebbcff550
    println!("address mag2: {:p}", &mag2); // address is 0x6ebbcff558
    println!("address mag3: {:p}", &mag3); // address is 0x6ebbcff568

    let mag4 = mag.clone();
    println!("address mag4: {:p}", &mag4); // address mag4: 0x7c0053f820

    //
    println!("mag is: {:?}", mag);
}

// fn return_magician<'a>() -> &'a Magician {
//   let mag = Magician { name: "Gandalf", power: 4625 };
//   &mag // error: `mag` does not live long enough
// }

// The value of n2 is 42, the same as n
// p is: 42
// address mag:  0x7c0053f708
// address mag2: 0x7c0053f710
// address mag3: 0x7c0053f720
// address mag4: 0x7c0053f820
// mag is: MagicNumber { value: 42 }


```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=b31638e943772afc35affe7fce30861e&version=stable)