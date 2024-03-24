```rust
#[allow(dead_code)]
enum Color {
   Red,
   Green,
   Blue,
   RGB(i32, i32, i32)
}

#[allow(dead_code)]
enum Color2 {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn print_color(c: Color) {
	match c {
		Color::Red => println!("#ff0000"),
		Color::Green => println!("#00ff00"),
		Color::Blue => println!("#0000ff"),
		Color::RGB(r, g, b) => println!("#{:02x}{:02x}{:02x}", r, g, b)
	}
}

#[allow(dead_code)]
enum Day {  // achterliggend discriminator vanaf 0
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}


impl Day {
    fn mood(&self) {
        println!("{}", match *self {
            Day::Friday => "it's friday!",
            Day::Saturday | Day::Sunday => "weekend :-)",
            _ => "weekday...",
        })
    }
}

fn main() {
	print_color(Color::Red);
	print_color(Color::RGB(0x12, 0x45, 0xba));

	// enums can be casted into integers
    println!("roses are #{:06x}", Color2::Red as i32);
    println!("violets are #{:06x}", Color2::Blue as i32);

    let today = Day::Monday;
    today.mood();
}
// #ff0000
// #1245ba
// roses are #ff0000
// violets are #0000ff
// weekday...
```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=421515b3479324b5588adef1b5fe5708&version=stable)
