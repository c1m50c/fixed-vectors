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

impl<T> From<(T, T)> for Vector2<T> {
    fn from(tuple: (T, T)) -> Self {
        return Self {
            x: tuple.0,
            y: tuple.1,
        };
    }
}

fn main() {
    let tuple = Vector2::new("Vector", "2").to_tuple();
    println!("Vector as Tuple: {:?}", tuple);
    assert_eq!(tuple, ("Vector", "2"));
}