#[cfg(test)]
mod tests;

#[cfg(feature = "macros")]
pub mod macros;
mod macros;


pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}


pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}


pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}


macros::impl_vector!(Vector2 { x, y }, 2);
macros::impl_vector!(Vector3 { x, y, z }, 3);
macros::impl_vector!(Vector4 { x, y, z, w}, 4);