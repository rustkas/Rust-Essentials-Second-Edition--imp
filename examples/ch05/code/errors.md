```rust
fn div(x: i32, y: i32) -> f32 {
    // if y == 0 { panic!("Division by 0 occurred, exiting"); }
    assert!(y != 0, "Division by 0 occurred, exiting");
    (x / y) as f32
}

#[allow(dead_code)]
fn div2(x: i32, y: i32) -> f32 {
    if y == 0 { panic!("Division by 0 occurred, exiting"); }
    // assert!(y != 0, "Division by 0 occurred, exiting");
    (x / y) as f32
}

#[allow(dead_code)]
fn ex01() {
    let x = 3;
    let y = 3;
    println!("{}", div(x, y)); // returns 1 if y = 3
}

#[allow(dead_code, unconditional_panic)]
fn ex02() {
    let x = 3;
    let y = 0;
    // let y = 3;
    let _ = x / y; // thread '<main>' panicked at 'attempted to divide by zero'
}

#[allow(dead_code)]
fn ex03() {

    let y = 0;
    if y == 0 { panic!("Division by 0 occurred, exiting"); }
}

#[allow(dead_code)]
fn ex04() {
    let x = 3;
    assert!(x == 5); // thread '<main>' panicked at 'assertion failed: x == 5'
}

#[allow(dead_code)]
fn ex05() {
    let x = 3;
    assert_eq!(x, 5); // thread '<main>' panicked at 'assertion failed: (left: `3`, right: `5`)',
}

#[allow(dead_code)]
fn ex06() {
    unreachable!(); // thread '<main>' panicked at 'internal error: entered unreachable code'
}

#[allow(dead_code)]
fn ex07() {
	let x = 3;
	// let y = 0;
	let y = 3;
	// x / y; // thread '<main>' panicked at 'attempted to divide by zero'
	// if y == 0 { panic!("Division by 0 occurred, exiting"); }
	println!("{}", div(x, y));  // returns 1 if y = 3    
}

#[allow(dead_code)]
fn ex08() {
	let x = 3;
	// let y = 0;
	let y = 3;
	// x / y; // thread '<main>' panicked at 'attempted to divide by zero'
	// if y == 0 { panic!("Division by 0 occurred, exiting"); }
	println!("{}", div2(x, y));  // returns 1 if y = 3    
}


fn main() {
    // ex01();
    // ex02();
    // ex03();
    // ex04();
    // ex05();
    // ex06();
    
    // ex07();
    ex08();
}


```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=a942a8cd662aa389c800058101916cf9&version=stable)
