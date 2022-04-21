# **fixed-vectors**

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/c1m50c/fixed-vectors/Build?style=for-the-badge)
![Crates.io](https://img.shields.io/crates/v/fixed-vectors?color=orange&style=for-the-badge)
![Crates.io](https://img.shields.io/crates/l/fixed-vectors?style=for-the-badge)
![Lines of code](https://img.shields.io/tokei/lines/github/c1m50c/fixed-vectors?style=for-the-badge)
![GitHub Repo stars](https://img.shields.io/github/stars/c1m50c/fixed-vectors?style=for-the-badge)

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

// StructName
// { StructFields, * }
// ( TupleGenerics, * )
// SizeOfVector
impl_vector!(Vector5 { x, y, z, w, v }, (T, T, T, T, T), 5);

fn main() {
    println!("Vector5 Name: {}", Vector5::<()>::NAME);
    println!("Vector5 Length: {}", Vector5::<()>::LEN);
    println!("Vector5<i32> Size: {}", Vector5::<i32>::SIZE);
    
    let vector = Vector5::new(1, 2, 3, 4, 5);

    println!("Vector: {}", vector);
    println!("Vector Debug: {:?}", vector);
    println!("Vector as Tuple: {:?}", vector.to_tuple());
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