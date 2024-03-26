```rust
fn increment(r: &mut isize) {
    *r += 1;
    println!("r is now {}", *r);
}

#[allow(unused_assignments, unused_variables, unused_mut)]
fn main() {
    // through the mutable pointer the value of x has been changed:
    let mut x = 5;
    increment(&mut x);
    println!("x is now {}", x);

    // borrowed references cannot inherit mutability:
    // mut borrowed means: the reference can change, but not its value!
    let mut val1 = 10;
    let mut val2 = 20;
    let mut borrowed = &mut val1;
    *borrowed = 11;

    // references are type checked:
    let mut val1 = 10;
    let mut val3 = 10.0;
    let mut borrowed = &mut val1;
    borrowed = &mut (val3 as i32);
}

// r is now 6
// x is now 6

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=0a2992abea93df4c062461c58c7c9450&version=stable)
