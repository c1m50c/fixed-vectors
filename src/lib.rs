pub mod macros;

#[cfg(test)]
mod tests;


pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}


macros::impl_vector!(Vector2 { x, y }, 2);