```rust
#[allow(dead_code)]
fn main() {
    println!("No tests are compiled, compile with rustc --test!");
    double(42);
}

pub fn double(n: i32) -> i32 {
    n * 2
}

#[test]
fn arithmetic() {
    // good tests:
    // assert_eq!(actual, expected)
    assert_eq!(5, 2 + 3);
    assert_eq!(double(42), 84);
    assert!(2 + 3 == 5);
    // bad tests:
    if 2 + 3 == 5 {
        println!("You can calculate!");
    }
    if 2 + 3 == 6 {
        // this test passes as wel!
        println!("You cannot calculate!");
    }
}

#[test]
#[should_panic(expected = "5")]
fn badtest() {
    assert_eq!(6, 2 + 3);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn failing_test() {
    assert!(6 == 2 + 3);
}

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=90757429289afd950f21ea697426a7bd&version=stable)
