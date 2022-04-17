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
    println!("Vector as Array: {:?}", vector.as_array());
    println!("Vector as Vec: {:?}", vector.as_vec());

    let mut sum = 0;
    for i in vector { sum += i; }
    println!("Vector Sum: {}", sum);
}