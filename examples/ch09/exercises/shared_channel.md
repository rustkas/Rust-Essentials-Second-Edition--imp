```rust
use std::sync::mpsc;
use std::thread;

static NTHREADS: usize = 8;

fn main() {
    let (tx, rx) = mpsc::channel();

    for id in 0..NTHREADS {
        let thread_tx = tx.clone(); // clone the sender end-point
        thread::spawn(move || {
            thread_tx.send(id).unwrap();
            println!("thread {} done", id);
        });
    }

    let mut ids = Vec::with_capacity(NTHREADS);
    for _ in 0..NTHREADS {
        ids.push(rx.recv().unwrap());
    }

    println!("{:?}", ids);
}
// -- The order is different each time the program is run: --
// thread 0 done
// thread 4 done
// thread 1 done
// thread 5 done
// thread 7 done
// thread 2 done
// thread 3 done
// thread 6 done
// [6, 1, 0, 4, 5, 7, 2, 3]


```
[Run in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=f36132c2e95f92895acdbdba53485a1b&version=stable)
