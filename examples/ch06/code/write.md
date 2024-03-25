```rust
use std::io::Write; // in order to be able to use write! on a vector

#[allow(unused_must_use)]
fn main() {
    let mut vec1 = Vec::new();
    write!(&mut vec1, "test").unwrap();
    println!("{:?}", vec1);
    
    // Преобразование Vec<u8> обратно в String
    let string_from_vec = String::from_utf8_lossy(&vec1);
    println!("{}", string_from_vec);

    // Преобразование Vec<u8> обратно в String, если вы уверены в корректности данных вектора
    let string_from_vec_strict = String::from_utf8(vec1).expect("Failed to convert Vec<u8> to String");
    println!("{}", string_from_vec_strict);
}
// [116, 101, 115, 116]

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=c8b230e60f262fce210373403c54a66d&version=stable)
