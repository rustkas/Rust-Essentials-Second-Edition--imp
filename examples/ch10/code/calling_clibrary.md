```rust
#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Complex {
    re: f32,
    im: f32,
}

#[link(name = "m")]
extern "C" {
    fn ctanf(z: Complex) -> Complex;
}

fn tan(z: Complex) -> Complex {
    unsafe { ctanf(z) }
}

fn main() {
    let z = Complex { re: -1., im: 1. }; // z is -1 + i
    let z_tan = tan(z);
    println!("the tangents of {:?} is {:?}", z, z_tan);
}
// the tangents of Complex { re: -1, im: 1 } is Complex { re: -0.271753, im: 1.083923 }

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=b608c391555d7c032906e83e95aa079d&version=stable)
