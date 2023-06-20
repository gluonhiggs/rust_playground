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
