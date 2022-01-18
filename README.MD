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
There are various examples implemented within the `examples` directory to look at for getting started. If you would like to run an example within the `examples` directory you can run the following commands.
```bash
$ cd fixed-vectors
$ cargo run --example [EXAMPLE_NAME]
```

### **Written Examples**
<details>
<summary><strong>Voxel</strong></summary>
Example below shows how a 
<a href="https://en.wikipedia.org/wiki/Voxel"><strong>Voxel</strong></a> 
might be implemented using a <strong>Vector3</strong>.

```rust
use fixed_vectors::Vector3;
use core::ops::Index;
use std::vec::Vec;

#[derive(Debug, PartialEq, Eq)]
struct Voxel {
    pub position: Vector3<u32>,
}

#[derive(Debug, PartialEq, Eq)]
struct VoxelChunk {
    voxels: Vec<Voxel>,
    size: Vector3<u32>,
}

impl VoxelChunk {
    pub fn new(size: Vector3<u32>) -> Self {
        let mut voxels = Vec::with_capacity((size.x * size.y * size.z) as usize);

        for x in 0 .. size.x {
            for y in 0 .. size.y {
                for z in 0 .. size.z {
                    let new_voxel = Voxel { position: Vector3::new(x, y, z) };
                    voxels.push(new_voxel);
                }
            }
        }

        return Self {
            voxels,
            size,
        };
    }
}

impl Index<Vector3<u32>> for VoxelChunk {
    type Output = Voxel;

    fn index(&self, index: Vector3<u32>) -> &Self::Output {
        return &self.voxels[position_as_index(self.size, index)];
    }
}

/*
    NOTE: Because of the order a [`VoxelChunk`] is constructed, we can use this formula to estimate an index
        of a [`Voxel`] within the `voxels` [`Vec`].
*/
const fn position_as_index(size: Vector3<u32>, position: Vector3<u32>) -> usize {
    return (position.z + (position.y * size.z) + (position.x * size.z * size.y)) as usize;
}

fn main() {
    let chunk = VoxelChunk::new(Vector3::new(8, 16, 8));
    let position = Vector3::<u32>::new(4, 8, 4);
    assert_eq!(chunk[position].position, position);
}
```
</details>

<details>
<summary><strong>Custom Vector</strong></summary>
Example below shows how you would create a custom <strong>Vector</strong> Struct.

```rust
use fixed_vectors::{Vector, impl_vector};

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

<details>
<summary><strong>Tuplable Vector</strong></summary>
Example below shows how you would implemented the <strong>TuplableVector</strong> Trait in a
<strong>Vector</strong>, as it's not currently implemented automatically with the <strong>impl_vector!</strong> Macro.

```rust
use fixed_vectors::{TuplableVector, impl_vector};

struct Vector2<T> {
    x: T,
    y: T,
}

impl_vector!(Vector2 { x, y }, 2);

impl<T> TuplableVector<T, { Vector2::<()>::LEN }> for Vector2<T> {
    type Output = (T, T);

    fn to_tuple(self) -> Self::Output {
        return (self.x, self.y);
    }
}

fn main() {
    let tuple = Vector2::new("Vector", "2").to_tuple();
    println!("Vector as Tuple: {:?}", tuple);
    assert_eq!(tuple, ("Vector", "2"));
}
```
</details>

---

## **License**
<a href="https://github.com/c1m50c/fixed-vectors/blob/main/LICENSE">MIT</a>