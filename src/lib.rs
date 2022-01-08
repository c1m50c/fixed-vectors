#[cfg(test)]
mod tests;


pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}


pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}


macro_rules! impl_vector {
    ($Vector:ident { $($field:ident), + }) => {
        impl<T> $Vector<T> {
            pub const fn new($($field: T), +) -> $Vector<T> {
                return $Vector {
                    $($field: $field), +
                };
            }
        }
    }
}


impl_vector!(Vector2 { x, y });
impl_vector!(Vector3 { x, y, z });