```rust
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

#[allow(dead_code)]
struct Info {
    name: String,
    age: i32,
    rating: i32,
}

impl Info {
    fn as_bytes(&self) -> &[u8] {
        self.name.as_bytes()
    }
    #[allow(dead_code)]
    fn format(&self) -> String {
        format!("{};{};{}\n", self.name, self.age, self.rating)
    }
}

#[allow(unused_variables)]
fn main() {
    let path = Path::new("info.txt");
    let display = path.display();

    let file = match write_file(&path) {
        Err(why) => panic!(
            "couldn't write info to file {}: {}",
            display,
            (&why.to_string())
        ),
        Ok(file) => file,
    };
}

fn write_file(path: &Path) -> Result<File, io::Error> {
    let mut file = (File::create(path))?;
    let info1 = Info {
        name: "Donald".to_string(),
        age: 71,
        rating: 7,
    };
    let info2 = Info {
        name: "Vladimir".to_string(),
        age: 58,
        rating: 6,
    };
    (file.write(info1.as_bytes()))?;
    (file.write(b"\r\n"))?;
    (file.write(info2.as_bytes()))?;
    Ok(file)
}

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=18f68656b1e0e8717c5ef7abbadc8ee9&version=stable)
