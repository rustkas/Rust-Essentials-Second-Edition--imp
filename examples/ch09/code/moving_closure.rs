```rust
use std::thread;

fn main() {
    read();
}

fn read() {
    let book = read_book("book1.txt");
    // error: closure may outlive the current function, but it borrows `book`,
    // which is owned by the current function
    // thread::spawn(|| {
    //     println!("{:?}", book);
    // });

    thread::spawn(move || {
        println!("{:?}", book);
    });
}

#[allow(unused_variables)]
fn read_book(s: &str) {}

// ()

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=736b813177901c6ecb28b1ba25a53e79&version=stable)
