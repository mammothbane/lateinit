# lateinit

[![Docs](https://docs.rs/lateinit/badge.svg)](https://docs.rs/lateinit)
[![Latest](https://img.shields.io/crates/v/lateinit.svg)](https://crates.io/crates/lateinit)

**Disclaimer**: this crate breaks Rust's safety guarantees. You should probably be using [`spin::Once`](https://mvdnes.github.io/rust-docs/spin-rs/spin/struct.Once.html), 
[`std::sync::Once`](https://doc.rust-lang.org/nightly/std/sync/struct.Once.html), or 
[`lazy_static`](https://github.com/rust-lang-nursery/lazy-static.rs) instead.

```toml
[dependencies]
lateinit = "0.2"
```

## Example usage
```rust
static SOMETHING: LateInit<String> = LateInit::new();

fn main() {
    let environment_value = std::env::var("ENV_VALUE").unwrap();
    unsafe { SOMETHING.init(environment_value); }
    
    println!("{}", SOMETHING);
}
```

## Design 

The `LateInit` type provides an unsafe interface for initializing static variables at runtime.
Design goals for this crate are to provide checked, one-time initialization and unchecked, zero-cost
access thereafter. The intention is to obviate the need for `static mut` in situations where only a 
single mutation is required for initialization purposes.

Methods like `is_initialized`, `init_once`, or similar are not and will not be supported because of the narrow
scope of these design goals. If you need to check whether a `LateInit` is initialized, you're using it incorrectly.

This crate should be used sparingly and carefully&mdash;it breaks safety because access is really `unsafe` 
despite not being marked as such, since `lateinit` provides no guarantees about whether a value is initialized
before it is used. It's on you (the programmer) to maintain this invariant. Bad things™ will 
happen if you don't. It's heavily suggested that you initialize only in a clearly demarcated region
of setup code.

## Features
`#[no_std]` is supported.

There are `debug_assert`s in trait implementations (most relevantly in `Deref`) to catch errors while testing. 
If you have performance concerns, you can turn off the `debug_assert`s with the `debug_unchecked` feature flag.
