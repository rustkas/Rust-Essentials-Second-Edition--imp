```rust
#[allow(unused_variables)]
fn main() {
    let score: i32 = 100;
    // score = 50; // re-assignment of immutable variable
   	// score = "YOU WON!";
	// error: mismatched types: expected i32, found reference - expected `i32`, found `&'static str` 
    let score = "YOU WON!";         

    let player1 = "Rob";
    let player2 = "Jane";
	// let player3 = player1 + player2;
	// error: binary operation `+` cannot be applied to type `&str`
    let player3 = player1.to_string() + player2;
    println!("{}", player3);
    let player3 = format!("{}{}", player1, player2);
    println!("{}", player3);
}
// RobJane
// RobJane
```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=c5d5534a155fe02ab0f2c6531e4a23fb&version=stable)
