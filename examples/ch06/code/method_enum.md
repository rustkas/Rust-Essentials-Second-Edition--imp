```rust
use Day::*;

#[allow(dead_code)]
enum Day {
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
        println!(
            "{}",
            match *self {
                Friday => "it's friday!",
                Saturday | Sunday => "weekend :-)",
                _ => "weekday...",
            }
        )
    }
}

fn main() {
    let today = Day::Tuesday;
    today.mood();

    for week_day in [
        Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday,
    ] {
        week_day.mood();
    }
}
// weekday...

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=c617e282aceaea48214aca9f50600c21&version=stable)
