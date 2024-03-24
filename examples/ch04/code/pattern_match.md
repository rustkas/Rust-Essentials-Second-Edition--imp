```rust
#use std::io;

fn main() {
    print!("Give a secret positive number: ");
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .ok()
        .expect("Failed to read number");
    let input_num: Result<u32, _> = buf.trim().parse();
    // alternative:
    // let input_num = buf.trim().parse::<u32>();

    // println!("Unwrap found {}", input_num.unwrap());

    match input_num {
        Ok(num) => println!("{}", num),
        Err(ex) => println!("\n\rPlease input an integer number! {}\n\r", ex),
    };

    // binding the value of a match:
    // let num = match input_num {
    //     Ok(num) => num,
    //     Err(_)  => 0
    // };

    let input_num: Result<u32, _> = buf.trim().parse();
    // alternative way for destructuring the Result:
    if let Ok(val) = input_num {
        println!("Matched {:?}!", val);
    } else {
        println!("No match!");
    }

    // while let Ok(val) = input_num {
    //     println!("Matched {:?}!", val);
    //     if val == 42 { break }
    // }
}
// Give a positive secret number: 42
// 42
// Matched 42!

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=46bae36da4c49a3a5a68a70aef2b2f1e&version=stable)
