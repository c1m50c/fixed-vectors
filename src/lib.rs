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


pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}


macro_rules! impl_vector {
    ($Vector:ident { $($field:ident), + }, $size:expr) => {
        impl<T> $Vector<T> {
            #[inline]
            pub const fn new($($field: T), +) -> $Vector<T> {
                return $Vector {
                    $($field: $field), +
                };
            }
        }

        impl<T: Default> Default for $Vector<T> {
            #[inline]
            fn default() -> $Vector<T> {
                return $Vector {
                    $($field: T::default()), +
                };
            }
        }
    }
}


impl_vector!(Vector2 { x, y }, 2);
impl_vector!(Vector3 { x, y, z }, 3);
impl_vector!(Vector4 { x, y, z, w }, 4);