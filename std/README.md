
The standard library exposes three common ways to deal with contiguous regions of memory:

- Vec<T> - A heap-allocated vector that is resizable at runtime.
- [T; n] - An inline array with a fixed size at compile time.
- [T] - A dynamically sized slice into any other kind of contiguous storage, whether heap-allocated or not.

Slices can only be handled through some kind of pointer, and as such come in many flavors such as:

- &[T] - shared slice
- &mut [T] - mutable slice
- Box<[T]> - owned slice

Rust's collections can be grouped into four major categories:

- Sequences: Vec, VecDeque, LinkedList
- Maps: HashMap, BTreeMap
- Sets: HashSet, BTreeSet
- Misc: BinaryHeap

