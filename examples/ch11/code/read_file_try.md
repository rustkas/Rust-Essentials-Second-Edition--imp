```rust
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io;

fn main() {
    let path = Path::new("hello.txt");
    let display = path.display(); 

    let content = match read_file(path) {
         Err(why) => panic!("error reading {}: {}", display, (&why.to_string())),
         Ok(content) => content
    };

    println!("{}", content);
}

fn read_file(path: &Path) -> Result<String, io::Error> {
    let mut file = (File::open(path))?;
    let mut buf = String::new();
    (file.read_to_string(&mut buf))?;
    Ok(buf)
}
// "Hello Rust World!" 
```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=99085e83d676298e0136362e0db64bc1&version=stable)
