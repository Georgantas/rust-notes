
https://doc.rust-lang.org/rust-by-example/

# 3. Enums
Thought: An enum is like a struct, but all the fields are OR'ed (only takes form of one of them) instead of AND'ed for structs (all fields are applied for that struct).

In patterns, `&` destructures a borrow, `ref` binds to a location by-reference rather than by-value. In other words, `&` lets you reach through a borrow, and ref says “take a borrow to this place within the thing I’m matching”.

# Flow of control
```rust
// What if you don't start with a reference? `reference` was a `&`
// because the right side was already a reference. This is not
// a reference because the right side is not one.
let _not_a_reference = 3;

// Rust provides `ref` for exactly this purpose. It modifies the
// assignment so that a reference is created for the element; this
// reference is assigned.
let ref _is_a_reference = 3;
```

The `@` vigil:
```rust
// A function `age` which returns a `u32`.
fn age() -> u32 {
    15
}

fn main() {
    println!("Tell me what type of person you are");

    match age() {
        0             => println!("I'm not born yet I guess"),
        // Could `match` 1 ..= 12 directly but then what age
        // would the child be? Instead, bind to `n` for the
        // sequence of 1 ..= 12. Now the age can be reported.
        n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        // Nothing bound. Return the result.
        n             => println!("I'm an old person of age {:?}", n),
    }
}
```

# 9. Functions
Closures try to borrow the variables by reference (like `&mut count` or `&count`). Exception:

```rust
// A non-copy type.
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };
```

To take closure in a function argument, use traits:
- Fn: the closure captures by reference (&T)
- FnMut: the closure captures by mutable reference (&mut T) (accepts Fn too)
- FnOnce: the closure captures by value (T) (accepts Fn and FnMuts too)

iter() and into_iter() for arrays, unusually, return references like &i32. Usually, its &i32 and i32, respectively for vectors for example.

`panic!` is a diverging function (ie. never returns, returns `!`) `let x: ! = panic!("This call never returns.");` They're useful for code like this:
```rust
fn main() {
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // Notice that the return type of this match expression must be u32
            // because of the type of the "addition" variable.
            let addition: u32 = match i%2 == 1 {
                // The "i" variable is of type u32, which is perfectly fine.
                true => i,
                // On the other hand, the "continue" expression does not return
                // u32, but it is still fine, because it never returns and therefore
                // does not violate the type requirements of the match expression.
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
}
```

# 10. Modules
```rust
// Functions declared using `pub(in path)` syntax are only visible
// within the given path. `path` must be a parent or ancestor module
pub(in crate::my_mod) fn public_function_in_my_mod() {
    print!("called `my_mod::nested::public_function_in_my_mod()`, that\n> ");
    public_function_in_nested();
}
```

Structs have an extra level of visibility with their fields. The visibility defaults to private, and can be overridden with the pub modifier. This visibility only matters when a struct is accessed from outside the module where it is defined, and has the goal of hiding information (encapsulation).

# 11. Crates
A crate is a compilation unit in Rust. Whenever rustc some_file.rs is called, some_file.rs is treated as the crate file. If some_file.rs has mod declarations in it, then the contents of the module files would be inserted in places where mod declarations in the crate file are found, before running the compiler over it. In other words, modules do not get compiled individually, only crates get compiled.

A crate can be compiled into a binary or into a library. By default, rustc will produce a binary from a crate. This behavior can be overridden by passing the --crate-type flag to lib.

# 12. Cargo
- build your project with `cargo build`
- run your project with `cargo run`
- test your project with `cargo test`
- build documentation for your project with `cargo doc`
- publish a library to crates.io with `cargo publish`

Cargo.toml file:
```
...
[dependencies]
clap = "2.27.1" # from crates.io
rand = { git = "https://github.com/rust-lang-nursery/rand" } # from online repo
bar = { path = "../bar" } # from a path in the local filesystem
```

Cargo may run multiple tests concurrently, so make sure that they don't race with each other. For example, if they all output to a file, you should make them write to different files.

Build scripts:
```
[package]
...
build = "build.rs"
```

Default script is build.rs in the project directory.

The build script is simply another Rust file that will be compiled and invoked prior to compiling anything else in the package.

Attributes: When attributes apply to a whole crate, their syntax is `#![crate_attribute]`, and when they apply to a module or item, the syntax is `#[item_attribute]` (notice the missing bang !).

Example attribute:
```rust
// This function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}
...
if cfg!(target_os = "linux") {
```

Pass custom conditionals with `--cfg` to `rustc`.

# 14. Generic
Look up `associated types` to simplify code like this:
```rust
// Without using associated types
fn difference<A, B, C>(container: &C) -> i32 where
    C: Contains<A, B> { ... }

// Using associated types
fn difference<C: Contains>(container: &C) -> i32 { ... }
```

It involves declaring `Contains` like this instead:
```rust

// `A` and `B` are defined in the trait via the `type` keyword.
// (Note: `type` in this context is different from `type` when used for
// aliases).
trait Contains {
    type A;
    type B;

    // Updated syntax to refer to these new types generically.
    fn contains(&self, &Self::A, &Self::B) -> bool;
}
```

Use `std::marker:::PhantomData` to decorate structs with data that doesn't actually reserve memory, but is only useful for compile-time checking.

# 15. Scoping Rules

Implement `Drop` to define a destructor.

Resources can only have one owner (example, a function can be the owner of a ressource):
```rust
// `a` is a pointer to a _heap_ allocated integer
let a = Box::new(5i32);

// *Move* `a` into `b`
let b = a;
// The pointer address of `a` is copied (not the data) into `b`.
// Both are now pointers to the same heap allocated data, but
// `b` now owns it.
```

All primitives implement the Copy trait:
```rust
// _Stack_ allocated integer
let x = 5u32;

// *Copy* `x` into `y` - no resources are moved
let y = x;
```
