Explanation:

Vec: The Vec<T> is Rust's dynamic array, capable of growing or shrinking. Here, it's used as the mutable collection to append items.

Ownership: The function takes vec as a mutable reference (&mut Vec<T>), transferring ownership of the new item into the vector. Rust's ownership rules ensure item is moved into vec, preventing double-free errors.

Borrowing: The &mut reference allows the function to modify vec without taking ownership, adhering to Rust's borrowing rules. This avoids cloning unless explicitly needed.

Implementation: The push method appends item to the end of vec, automatically handling memory reallocation if necessary.

Notes:

This function is generic over T, making it work with any type, leveraging Rust's type system.

The main function demonstrates usage with integers, but it can handle any type (e.g., String, custom structs) as long as ownership is properly managed.

Error handling (e.g., for memory exhaustion) is omitted for simplicity but can be added with try_reserve or similar in a production context.
