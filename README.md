# shelle

[![Crates.io](https://img.shields.io/crates/v/shelle.svg)](https://crates.io/crates/shelle)
[![License: MIT](https://img.shields.io/badge/License-MIT-orange.svg)](https://opensource.org/licenses/MIT)

Macros for writing shell scripts in Rust.

This project is based on [cmd_lib](https://github.com/rust-shell-script/rust_cmd_lib). Thanks to @tao-guo and other contributors for your hard work.

# Usage

`shelle::exec!()` runs command(s) with stdin/stdout/stderr inherited from the main program:

```rust
let msg = "I love rust";
shelle::exec!(echo #msg)?;
shelle::exec!(echo "This is the message: #msg")?;

// pipe commands are also supported
let dir = "/var/log";
shelle::exec!(du -ah #dir | sort -hr | head -n 10)?;

// or a group of commands
// if any command fails, just return Err(...)
let file = "/tmp/f";
let keyword = "rust";
shelle::exec! {
    cat #{file} | grep #{keyword};
    echo "bad cmd" >&2;
    ignore ls /nofile;
    date;
    ls oops;
    cat oops;
}?;
```

`shelle::eval!()` runs command(s) with stdout piped to a string, and stdin/stderr inherited from the main program:

```rust
let version = shelle::eval!(rustc --version)?;
println!("Your rust version is {}", version);

// with pipes
let n = shelle::eval!(echo "the quick brown fox jumped over the lazy dog" | wc -w)?;
println!("There are {} words in above sentence", n);
```
