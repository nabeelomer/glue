# Glue

**[WIP]** Glue is a library that provides the _glue_ to write Ruby code inside a Rust program.

```rust
use glue::*;

fn main() {
    ruby! {
        puts "Hello World from Ruby across the bridge!"
    };
}
```
