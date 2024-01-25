# Rust
Rust is a multi-paradigm, general-purpose programming language. Rust emphasizes performance, type safety, and concurrency. Rust enforces memory safety—that is, that all references point to valid memory—without requiring the use of a garbage collector or reference counting present in other memory-safe languages.

## Features 
Rust aims to support concurrent systems programming, which has inspired a feature set with an emphasis on safety, control of memory layout, and concurrency.

### Memory safety
Rust is designed to be memory safe. It does not permit null pointers, dangling pointers, or data races. Data values can be initialized only through a fixed set of forms, all of which require their inputs to be already initialized. To replicate pointers being either valid or `NULL`, such as in linked list or binary tree data structures, the Rust core library provides an option type, which can be used to test whether a pointer has Some value or None. Rust has added syntax to manage lifetimes, which are checked at compile time by the borrow checker. Unsafe code can subvert some of these restrictions using the `unsafe` keyword. Unsafe code may also be used for low-level functionality like volatile memory access, architecture-specific intrinsics, type punning, and inline assembly.

### Memory management
Rust does not use automated garbage collection. Memory and other resources are managed through the resource acquisition in initialization convention, with optional reference counting. Rust provides deterministic management of resources, with very low overhead. Values are allocated on the stack by default and all dynamic allocations must be explicit.

The built-in reference types using the `&` symbol do not involve run-time reference counting. The safety and validity of the underlying pointers is verified at compile time, preventing dangling pointers and other forms of undefined behavior. Rust's type system separates shared, immutable references of the form `&T` from unique, mutable references of the form `&mut T`. A mutable reference can be coerced to an immutable reference, but not vice versa.

### Types and polymorphism
Rust's type system supports a mechanism called traits, inspired by type classes in the Haskell language. Traits annotate types and are used to define shared behavior between different types. For example, floats and integers both implement the `Add` trait because they can both be added; and any type that can be printed out as a string implements the `Display` or `Debug traits`. This facility is known as *ad hoc polymorphism*.

In Rust, user-defined types are created with the struct or enum keywords. The struct keyword denotes a record type. `Enums` are kinds of algebraic data types commonly found in functional programming languages. These types can contain fields of other types. The `impl` keyword can define methods for the types (data and functions are defined separately) or implement a trait for the types. Traits are used to restrict generic parameters and because traits can provide a type with more methods than the user defined. For example, the trait `Iterator` requires that the `next` method be defined for the type. Once the `next` method is defined the trait provides common functional helper methods over the iterator like `map` or `filter`.

Type aliases, including generic arguments, can also be defined with the *type* keyword.

### Macros
It is possible to extend the Rust language using macros.

#### Declarative macros
Declarative macros (also called "macros by example") are macros that uses pattern matching to determine its expansion.

#### Procedural macros
Procedural macros use Rust functions that are compiled before other components to run and modify the compiler's input token stream. They are generally more flexible than declarative macros, but are more difficult to maintain due to their complexity.

Procedural macros comes in three flavors:
* Function-like macros `custom!(...)`
* Derive macros `#[derive(CustomDerive)]`
* Attribute macros `#[custom_attribute]`

#### Interface with C and C++
Rust has a foreign function interface (FFI) that can be used both to call code written in languages such as C from Rust and to call Rust code from those languages. Rust also has a library, CXX, for calling to or from C++. Rust and C differ in how they lay out structs in memory, so Rust structs may be given a `#[repr(C)]` attribute, forcing the same layout as the equivalent C struct.
