# **fixed-vectors**

[![GitHub - Publish Status](https://img.shields.io/github/workflow/status/c1m50c/fixed-vectors/Publish?style=for-the-badge)](https://github.com/c1m50c/fixed-vectors/actions)
[![Docs.rs - Latest](https://img.shields.io/docsrs/fixed-vectors?style=for-the-badge)](https://docs.rs/fixed-vectors/latest/fixed_vectors/)
[![Crates.io - Downloads](https://img.shields.io/crates/d/fixed-vectors?style=for-the-badge)](https://crates.io/crates/fixed-vectors/)
[![Crates.io - Version](https://img.shields.io/crates/v/fixed-vectors?color=orange&style=for-the-badge)](https://crates.io/crates/fixed-vectors/)
[![Crates.io - License](https://img.shields.io/crates/l/fixed-vectors?style=for-the-badge)](https://crates.io/crates/fixed-vectors/)

Library implementing fixed-length vectors meant for representing positional and dimensional values. These vectors have various mathematical and helper functions implemented for them out of the box for ease of use in calculations, especially game development related ones. These methods are all implemented using the [`impl_vector!`](https://github.com/c1m50c/fixed-vectors/blob/main/src/macros/mod.rs) macro.

# Example

```rust
use fixed_vectors::Vector2;
use std::convert::TryFrom;

let str_vec = Vector2::new("Hello", "World");
assert_eq!(str_vec.to_tuple(), ("Hello", "World"));

let mut num_vec = Vector2::new(1.0, 2.0);
num_vec += Vector2::new(0.4, 0.9);
assert_eq!(num_vec.round(), Vector2::new(1.0, 3.0));

if let Ok(from_vec_vec) = Vector2::try_from(vec![4, 2]) {
    assert_eq!(from_vec_vec, Vector2::new(4, 2));
}
```