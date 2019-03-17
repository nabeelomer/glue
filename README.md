# Glue

**[WIP]** Glue is a library that provides the _glue_ to call Ruby code from a Rust program.

```rust
use glue::*;

fn main() {
    ruby! {
        puts "Hello World from Ruby across the bridge!"
    };
}
```
