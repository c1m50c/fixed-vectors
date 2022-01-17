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
    pub position: Vector3<i32>,
}
```
</details>

<details>
<summary><strong>Custom Vector</strong></summary>
Example below shows how you would create a custom <strong>Vector</strong> Struct.

```rust
use fixed_vectors::impl_vector;

struct Vector5<T> {
    x: T,
    y: T,
    z: T,
    w: T,
    v: T,
}

impl_vector!(Vector5 { x, y, z, w, v }, 5);

fn main() {
    println!("Vector5 Name: {}", Vector5::<()>::NAME);
    println!("Vector5 Length: {}", Vector5::<()>::LEN);
    println!("Vector5<i32> Size: {}", Vector5::<i32>::SIZE);
    
    let vector = Vector5::new(1, 2, 3, 4, 5);

    println!("Vector: {}", vector);
    println!("Vector Debug: {:?}", vector);
    println!("Vector as Array: {:?}", vector.to_array());
    println!("Vector as Vec: {:?}", vector.to_vec());

    let mut sum = 0;
    for i in vector { sum += i; }
    println!("Vector Sum: {}", sum);
}
```
</details>

---

## **License**
<a href="https://github.com/c1m50c/fixed-vectors/blob/main/LICENSE">MIT</a>