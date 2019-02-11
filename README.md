# SSSSTD 🐍

The name Vector for a resizeable array of values easily causes confusion with mathematical vectors, snakes are a much better analogy for such a structure.

- A snake occupies a finite continuous region of space
- As a snake outgrows its capacity, it has to re-allocate. Biologists tend to refer to this as shedding skin.


## Usage

```rust
use sssstd::snek::Snek;

fn main() {
    let snek = Snek::<i32>::new();
    snek.push(1);
    println!("{}", snek);
}
```
