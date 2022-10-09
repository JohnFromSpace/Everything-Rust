# Rust
Rust is a multi-paradigm, general-purpose programming language. Rust emphasizes performance, type safety, and concurrency. Rust enforces memory safety—that is, that all references point to valid memory—without requiring the use of a garbage collector or reference counting present in other memory-safe languages.

## Features 
Rust aims to support concurrent systems programming, which has inspired a feature set with an emphasis on safety, control of memory layout, and concurrency.

### Memory safety
Rust is designed to be memory safe. It does not permit null pointers, dangling pointers, or data races. Data values can be initialized only through a fixed set of forms, all of which require their inputs to be already initialized. To replicate pointers being either valid or NULL, such as in linked list or binary tree data structures, the Rust core library provides an option type, which can be used to test whether a pointer has Some value or None. Rust has added syntax to manage lifetimes, which are checked at compile time by the borrow checker. Unsafe code can subvert some of these restrictions using the unsafe keyword. Unsafe code may also be used for low-level functionality like volatile memory access, architecture-specific intrinsics, type punning, and inline assembly.

### Memory management
Rust does not use automated garbage collection. Memory and other resources are managed through the resource acquisition in initialization convention, with optional reference counting. Rust provides deterministic management of resources, with very low overhead. Values are allocated on the stack by default and all dynamic allocations must be explicit.

The built-in reference types using the & symbol do not involve run-time reference counting. The safety and validity of the underlying pointers is verified at compile time, preventing dangling pointers and other forms of undefined behavior. Rust's type system separates shared, immutable references of the form &T from unique, mutable references of the form &mut T. A mutable reference can be coerced to an immutable reference, but not vice versa.

