```rust
fn increment(mut v: Vec<i32>) -> Vec<i32> {
    for i in 0..v.len() {
        v[i] += 1;
    }
    v
}

fn increment_mut(v: &mut Vec<i32>) {
    for i in 0..v.len() {
        v[i] += 1;
    }
}

fn main() {
    let p = vec![1, 2, 3];
    let q = increment(p);
    print!("new vector: ");
    for x in q {
        print!("{} ", x);
    }
    println!("");

    let mut p = vec![1, 2, 3];
    increment_mut(&mut p);
    print!("change in place: ");
    for x in p {
        print!("{} ", x);
    }
}

// new vector: 2 3 4
// change in place: 2 3 4

```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=56b88b29ef3700faecee69f8e489817e&version=stable)
