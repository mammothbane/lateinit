# lateinit

**Disclaimer**: this crate breaks Rust's safety guarantees. You should probably be using [`spin::Once`](https://mvdnes.github.io/rust-docs/spin-rs/spin/struct.Once.html), 
[`std::sync::Once`](https://doc.rust-lang.org/nightly/std/sync/struct.Once.html), or 
[`lazy_static`](https://github.com/rust-lang-nursery/lazy-static.rs) instead. If you're sure you 
want zero-cost late initialization at the cost of safety, read on.

```toml
[dependencies]
lateinit = "0.1"
```

The `LateInit` type provides an unsafe interface for initializing static variables at runtime.
Design goals for this crate are to provide checked, one-time initialization and unchecked, zero-cost
access thereafter.

```rust
static SOMETHING: LateInit<String> = LateInit::new();

fn main() {
    let environment_value = std::env::var("ENV_VALUE").unwrap();
    unsafe { SOMETHING.init(environment_value); }
    
    println!("{}", SOMETHING);
}
```

This crate should be used sparingly and carefully&mdash;it breaks Rust's safety guarantees.
It's on you to ensure that you initialize your `LateInit`s before accessing. Bad things™ will 
happen otherwise. It's heavily suggested that you initialize only in a clearly demarcated region
of setup code.

## Features
`#[no_std]` is supported.

By default, `init` asserts that it hasn't been called before, and there are `debug_assert`s in
the `Deref` and `AsRef` implementations to catch potential errors while testing. If for some
reason you want to turn these safety measures off, you can enable the `unchecked` feature flag 
and they will be compiled out.
