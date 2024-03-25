```rust
use std::io;
use std::error;


fn input_num() -> Result<u32, Box<dyn error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().parse()?)
}


// try! macro
fn input_num2() -> Result<u32, Box<dyn error::Error>> {
    let mut input = String::new();
    // try!(io::stdin().read_line(&mut input));
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().parse()?)
}

// error propagation operator ?


fn main() {
    println!("Give a positive secret number: ");
    match input_num() {
        Ok(v) => println!("Input value is: {}", v),
        Err(e) => println!("Error - Please input an integer number!: {}", e)
    }
    
    println!("Give a positive secret number: ");
    match input_num2() {
        Ok(v) => println!("Input value is: {}", v),
        Err(e) => println!("Error - Please input an integer number!: {}", e)
    }
}

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=fe8824d07e19a4629de58155470aac43&version=stable)
