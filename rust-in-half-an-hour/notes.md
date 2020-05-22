
https://fasterthanli.me/blog/2020/a-half-hour-to-learn-rust/

Rust inserts this at the beginning of every module:
```rust
use std::prelude::v1::*;
```
(Which in turns re-exports a lot of symbols, like `Vec`, `String`, `Option` and `Result`).

You can implement:
- one of your traits on anyone's type
- anyone's trait on one of your types
- but not a foreign trait on a foreign type

Some traits are so common, they can be implemented automatically by using the derive attribute:
```rust
#[derive(Clone, Copy)]
struct Number {
    odd: bool,
    value: i32,
}

// this expands to `impl Clone for Number` and `impl Copy for Number` blocks.
```

The simplest constraints are just trait names:

```rust
fn print<T: Display>(value: T) {
    println!("value = {}", value);
}

fn print<T: Debug>(value: T) {
    println!("value = {:?}", value);
}
```

Longer syntax:
```rust
fn print<T>(value: T)
where
    T: Display,
{
    println!("value = {}", value);
}
```

Same as with crates, and modules, and types, generic functions can be “explored” (navigated?) using ::

```rust
fn main() {
    use std::any::type_name;
    println!("{}", type_name::<i32>()); // prints "i32"
    println!("{}", type_name::<(f64, char)>()); // prints "(f64, char)"
}
```

This is lovingly called turbofish syntax, because ::<> looks like a fish.

Generic struct:
```rust
struct Pair<T> {
    a: T,
    b: T,
}
```

All of `name!()`, `name![]` or `name!{}` invoke a macro. Macros just expand to regular code.

Named lifetimes:

`fn foo<'a, T>(t: &'a T) -> &'a T` says that foo returns a pointer that has the same lifetime as t, that is, the data it points to is valid for the same length of time as t (well, strictly, at least as long as). 
