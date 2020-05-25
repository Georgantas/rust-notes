
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

Usefulness of `ref`. Kind of works as a `get the reference of` operator when destructuring data:
```rust
#[derive(Clone, Copy)]
struct Point { x: i32, y: i32 }

fn main() {
    let c = 'Q';

    // A `ref` borrow on the left side of an assignment is equivalent to
    // an `&` borrow on the right side.
    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point { x: 0, y: 0 };

    // `ref` is also valid when destructuring a struct.
    let _copy_of_x = {
        // `ref_to_x` is a reference to the `x` field of `point`.
        let Point { x: ref ref_to_x, y: _ } = point;

        // Return a copy of the `x` field of `point`.
        *ref_to_x
    };

    // A mutable copy of `point`
    let mut mutable_point = point;

    {
        // `ref` can be paired with `mut` to take mutable references.
        let Point { x: _, y: ref mut mut_ref_to_y } = mutable_point;

        // Mutate the `y` field of `mutable_point` via a mutable reference.
        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y);

    // A mutable tuple that includes a pointer
    let mut mutable_tuple = (Box::new(5u32), 3u32);
    
    {
        // Destructure `mutable_tuple` to change the value of `last`.
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }
    
    println!("tuple is {:?}", mutable_tuple);
}
```

Annotating the lifetimes **of references** explicitly:
```rust
// `print_refs` takes two references to `i32` which have different
// lifetimes `'a` and `'b`. These two lifetimes must both be at
// least as long as the function `print_refs`.
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// A function which takes no arguments, but has a lifetime parameter `'a`.
fn failed_borrow<'a>() {
    let _x = 12;

    // ERROR: `_x` does not live long enough
    //let y: &'a i32 = &_x;
    // Attempting to use the lifetime `'a` as an explicit type annotation 
    // inside the function will fail because the lifetime of `&_x` is shorter
    // than that of `y`. A short lifetime cannot be coerced into a longer one.
}

fn main() {
    // Create variables to be borrowed below.
    let (four, nine) = (4, 9);
    
    // Borrows (`&`) of both variables are passed into the function.
    print_refs(&four, &nine);
    // Any input which is borrowed must outlive the borrower. 
    // In other words, the lifetime of `four` and `nine` must 
    // be longer than that of `print_refs`.
    
    failed_borrow();
    // `failed_borrow` contains no references to force `'a` to be 
    // longer than the lifetime of the function, but `'a` is longer.
    // Because the lifetime is never constrained, it defaults to `'static`.
}
```

`T` is bounded to a lifetime:
```rust
// `Ref` contains a reference to a generic type `T` that has
// an unknown lifetime `'a`. `T` is bounded such that any
// *references* in `T` must outlive `'a`. Additionally, the lifetime
// of `Ref` may not exceed `'a`.
#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);

// `<'a: 'b, 'b>` reads as lifetime `'a` is at least as long as `'b`.
// Here, we take in an `&'a i32` and return a `&'b i32` as a result of coercion.
// Thought: similar to casting? Can cast to shorter lifetime.
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

fn main() {
    let first = 2; // Longer lifetime
    
    {
        let second = 3; // Shorter lifetime
        
        println!("{} is the first", choose_first(&first, &second));
    };
}
```

A `'static` lifetime is the longest possible lifetime, and lasts for the lifetime of the running program. There are two ways to make a variable with 'static lifetime, and both are stored in the read-only memory of the binary:
- Make a constant with the static declaration.
- Make a string literal which has type: &'static str.

Example:
```rust
// Make a constant with `'static` lifetime.
static NUM: i32 = 18;

// Returns a reference to `NUM` where its `'static` 
// lifetime is coerced to that of the input argument.
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        // Make a `string` literal and print it:
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // When `static_string` goes out of scope, the reference
        // can no longer be used, but the data remains in the binary.
    }
    
    {
        // Make an integer to use for `coerce_static`:
        let lifetime_num = 9;

        // Coerce `NUM` to lifetime of `lifetime_num`:
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }
    
    println!("NUM: {} stays accessible!", NUM);
}
```

Elision: Some lifetime patterns are overwhelmingly common and so the borrow checker will allow you to omit them to save typing and to improve readability.

# 16. Traits
Returning traits: Rust tries to be as explicit as possible whenever it allocates memory on the heap. So if your function returns a pointer-to-trait-on-heap in this way, you need to write the return type with the dyn keyword, e.g. Box<dyn Animal>.

If your function returns a type that implements MyTrait, you can write its return type as `-> impl MyTrait`.

`dyn` vs `impl` in return position:
- dyn is chosen at run time and has runtime overhead. You have a lot of freedom in what underlying type is actually returned.
- impl (in return position) is chosen at compile time and allows more optimized code. It’s just a syntax sugar for hardcoding a single type.

Rust doesn't have "inheritance", but you can define a trait as being a superset of another trait:
```rust
// CompSciStudent (computer science student) is a supertrait of both Programmer 
// and Student. Implementing CompSciStudent requires you to impl both subtraits.
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}
```

If you implement two traits that have a function with the same name, use the fully qualified syntax to disambiguate between the two:
```rust
let username = <Form as UsernameWidget>::get(&form);
```

Rust macros are expanded into abstract syntax trees, rather than string preprocessing, so you don't get unexpected precedence bugs.

# 17. Macros
Good for: Not repeating yourself. Domain specific language (special syntax). Variadic interfaces.

Macro designators: `macro_rules! create_function { ($func_name:ident) => { ...` here `$func_name` is the variable name for the macro and `ident` is the designator which designates variable/function names.

Overload: a macro is like a match statement.

Repeat: You can repeat an argument with `+` or `*`. `($x:expr, $($y:expr),+) => (`

# 19. Std Library Types
All values in Rust are stack allocated by default. Values can be boxed (allocated on the heap) by creating a `Box<T>`.

Vectors are re-sizable arrays. Like slices, their size is not known at compile time, but they can grow or shrink at any time.

`String` and `&str`. `Options`. `Result`. Byte string/array: `[u8; 21]`. `HashMap`. `HashSet` (wrapper to `HashMap<T, ()>`).

# 20. Std Misc
`std::thread`. Asynchronous `channels` for talking between threads (basically a queue?). `std::path::Path`. `std::process`. `std::fs::File`.

# 21. Testing
Unit tests: Unit tests are testing one module in isolation at a time: they're small and can test private code. Most unit tests go into a tests mod with the `#[cfg(test)]` attribute. Test functions are marked with the `#[test]` attribute. `#[ignore]` to skip a test.

Documentation tests: Runs code from the comments. Examples:
```rust
/// Using hidden `try_main` in doc tests.
///
/// ```
/// # // hidden lines start with `#` symbol, but they're still compileable!
/// # fn try_main() -> Result<(), String> { // line that wraps the body shown in doc
/// let res = try::try_div(10, 2)?;
/// # Ok(()) // returning from try_main
/// # }
/// # fn main() { // starting main that'll unwrap()
/// #    try_main().unwrap(); // calling try_main and unwrapping
/// #                         // so that test will panic in case of error
/// # }
pub fn try_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Divide-by-zero"))
    } else {
        Ok(a / b)
    }
}
```
```rust
/// First line is a short summary describing function.
///
/// The next lines present detailed documentation. Code blocks start with
/// triple backquotes and have implicit `fn main()` inside
/// and `extern crate <cratename>`. Assume we're testing `doccomments` crate:
///
/// ```
/// let result = doccomments::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Integration tests: Cargo looks for integration tests in tests directory next to src. Use to test the public interface of the library. Each Rust source file in tests directory is compiled as a separate crate. Modules with common code follow the ordinary modules rules, so it's ok to create common module as tests/common/mod.rs.

To have dependencies for tests, add them to Cargo.toml in the [dev-dependencies] section.

# 22. Unsafe Operations
Raw pointers * and references &T function similarly, but references are always safe because they are guaranteed to point to valid data due to the borrow checker. Dereferencing a raw pointer can only be done through an unsafe block.
```rust
fn main() {
    let raw_p: *const u32 = &10;

    unsafe {
        assert!(*raw_p == 10);
    }
}
```

# 23. Meta
Documentation lines support markdown.
