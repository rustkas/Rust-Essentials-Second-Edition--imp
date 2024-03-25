```rust
use std::f32;

fn sqroot(r: f32) -> Result<f32, String> {
    if r < 0.0 {
        return Err("Number cannot be negative!".to_string());
    }
    // assert!(r >= 0.0, "Number cannot be negative!");
    Ok(f32::sqrt(r))
}

fn main() {
    // using Result<T, E>
    let m = sqroot(42.0);
    
    let res = match m {
        Ok(sq) => sq,
        Err(_) => -1.0,
    };
    println!("res is {:.2}", res);
    
    let m = sqroot(-5.0);

    match m {
     Ok(sq) => println!("The square root of 42 is {}", sq),
     Err(str) => println!("{}", str)
    }

    
}

// The square root of 42 is 6.480741
// mres is 6.480741
// for m == -5.0: Number cannot be negative!
// res is 6.4807405

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=2038d904545d30a5c2fe98b6e99c865d&version=stable)
