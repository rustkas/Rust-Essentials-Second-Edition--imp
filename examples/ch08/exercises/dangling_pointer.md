```rust
#[allow(dead_code)]
struct IntNumber<'a> {
    x: &'a i32,
}

#[allow(dead_code,unused_assignments,unused_mut,unused_variables)]
fn main() {
	// n does not live long enough to be assigned to m
    // let m: &u32 = { 
    //      let n = &5u32; // error: borrowed value does not live long enough
    //      n
    // }; 
    // let o = *m;   

    // The following will be rejected, since y has a shorter lifetime than x.
    let mut x = &3;
    {
	    let mut y = 4;
// 		x = &y; // error: `y` does not live long enough
    } // y is freed here, but x still lives...
    println!("{}", x);
    
    // 
    let mut x = 1;                    // -+ x goes into scope
                              //  |
    {                         //  |
        let y = &5;           // ---+ y goes into scope
        let f = IntNumber { x: y }; // ---+ f goes into scope
        x = **&f.x;             //  | | error here
    }                         // ---+ f and y go out of scope
                              //  |
    println!("{}", x);        //  |
}
```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=49d4dbbefe1882eeee9a55520c2a019a&version=stable)
