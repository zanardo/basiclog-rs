# basiclog

This is a simple Rust logging boilerplate for my needs.

Importing:

```rust
use basiclog::{info};
```

Just initialize it:

```rust
basiclog::init();
```

And use:

```rust
info!("This is a test!");
```

Output:

```
2021/04/28 21:57:04.331 DEB | This is a test!
```
