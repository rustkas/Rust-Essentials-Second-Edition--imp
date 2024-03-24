```rust
#[allow(unused_variables, unused_assignments, unused_mut)]
fn main() {
    let mut a = 5;
    let mut b = 6;
    let n = 7;
    
    let a = b = n;
    println!("a = {:?}, b = {:?}, c = {:?}", a, b, n); // a = (), b = 7, c = 7

    // no swap :
    let mut c = 5;
    let mut d = 6;
    let c = d = c;
    println!("c = {:?}, d = {:?}", c, d);  // ()5
}
// a gets the value of the expression:  b = n;
// the value of that expression is ()
// a = (), b = 7, c = 7
// c = (), d = 5


```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=41ad6b3be82ac9d43290f3a4dc7ca92d&version=stable)
