## Prerequisites

### Data Types

#### Integer
In general, the difference between unsigned and signed integers has to do with whether they can represent negative numbers or not.

- Signed integers can represent both positive and negative numbers. This is accomplished by designating one bit (usually the leftmost, or most significant bit) as the sign bit. If the sign bit is 0, the number is positive. If it's 1, the number is negative.

For example, let's consider an 8-bit signed integer. This type of integer can represent numbers from -128 to 127, because one bit is used for the sign.

- Unsigned integers, on the other hand, can only represent non-negative numbers (i.e., zero and positive numbers). All the bits in an unsigned integer are used to denote the magnitude of the number.

For example, an 8-bit unsigned integer can represent numbers from 0 to 255. All 8 bits are used to denote the magnitude of the number.

The situation in Rust is similar but with specific nuances. Rust provides several built-in types for signed and unsigned integers, with explicit sizes:

- Signed integers: i8, i16, i32, i64, i128, isize. The number following the "i" indicates the number of bits.
- Unsigned integers: u8, u16, u32, u64, u128, usize. The number following the "u" indicates the number of bits.

Here are examples of defining signed and unsigned integers in Rust:

```rust
let x: i32 = -100;  // a signed 32-bit integer
let y: u32 = 100;  // an unsigned 32-bit integer
```
#### Floating-Point
Floating-point numbers are numbers with a fractional component, such as 3.14, 1.618, or 0.1. In Rust, there are two types of floating-point numbers: f32 and f64, which are 32 bits and 64 bits in size, respectively. The default type is f64 because on modern CPUs it's roughly the same speed as f32 but is capable of more precision.

Here are examples of defining floating-point numbers in Rust:

```rust
let x: f32 = 3.14;  // a 32-bit floating-point number
let y: f64 = 1.618;  // a 64-bit floating-point number
```
#### Boolean
Boolean values represent truth values. They can either be true or false. In Rust, the type of a boolean value is bool.

Here are examples of defining boolean values in Rust:

```rust
let x = true;
let y: bool = false; // with explicit type annotation
```
#### Character
A character is a single Unicode scalar value. In Rust, the type of a character is char, and a char value is always 4 bytes in size.

Here are examples of defining characters in Rust:

```rust
let x = 'x';
let y: char = 'y'; // with explicit type annotation
```
