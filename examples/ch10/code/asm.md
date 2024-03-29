```rust
use core::arch::asm;

fn subtract(a: i32, b: i32) -> i32 {
    let mut sub = 0;
    unsafe {
        asm!("sub {0:e}, {1:e}; mov {2:e}, {0:e}",
            in(reg) a,
            in(reg) b,
            inout(reg) sub
        );
    }
    if sub < 0 {
        sub = i32::MAX - sub + 1;
    }

    sub
}

fn main() {
    println!("{}", subtract(42, 7)); // 35
}

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=b29dbe34bf9be87293d158a637eeba28&version=stable)

