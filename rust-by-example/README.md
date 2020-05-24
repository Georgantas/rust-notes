
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

# 12. Cargo
- build your project with `cargo build`
- run your project with `cargo run`
- test your project with `cargo test`
- build documentation for your project with `cargo doc`
- publish a library to crates.io with `cargo publish`
