# Modifying a Vector While Iterating in Rust
This repository demonstrates a common error in Rust: modifying a vector while iterating over it using a range-based for loop.  This can lead to unexpected behavior or even a panic.

The `bug.rs` file contains the buggy code. The `bugSolution.rs` file shows how to correctly iterate and modify a vector without causing errors.

This example highlights the importance of careful consideration when modifying data structures during iteration in Rust.