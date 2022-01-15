# **fixed-vectors**

[![Build](https://github.com/c1m50c/fixed-vectors/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/c1m50c/fixed-vectors/actions/workflows/build.yml)

Library implementing fixed-length Vectors meant for representing dimensional values. Tested heavily to ensure safety during use.

---

## **Testing**
```bash
$ cd fixed-vectors
$ cargo test
...
# If things go well during the tests you should see `ok` as the test result.
```

---

## **Examples**

<details>
<summary><strong>Voxel Struct</strong></summary>
Example below shows how a 
<a href="https://en.wikipedia.org/wiki/Voxel"><strong>Voxel</strong></a> 
might be represented as a struct,
using a <strong>Vector3</strong>.

```rust
use fixed_vectors::Vector3;

pub struct Voxel {
    /// Represents the positional value in 3D Space of the [`Voxel`]
    pub position: Vector<i32>,
}
```
</details>

---

## **License**
<a href="https://github.com/c1m50c/fixed-vectors/blob/main/LICENSE">MIT</a>