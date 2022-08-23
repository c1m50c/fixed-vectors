#[cfg(test)]
mod tests;

#[cfg(feature = "macros")]
pub mod macros;
mod macros;


/// A Vector for holding two dimensional-values.
/// 
/// # Example
/// ```rust
/// use fixed_vectors::Vector2;
/// 
/// let vec2 = Vector2::new(1, 2);
/// assert_eq!(vec2.x, 1);
/// assert_eq!(vec2.y, 2);
/// ```
pub struct Vector2<T> {
    /// Field holding the first dimensional-value.
    pub x: T,

    /// Field holding the second dimensional-value.
    pub y: T,
}


/// A Vector for holding three dimensional-values.
/// 
/// # Example
/// ```rust
/// use fixed_vectors::Vector3;
/// 
/// let vec3 = Vector3::new(1, 2, 3);
/// assert_eq!(vec3.x, 1);
/// assert_eq!(vec3.y, 2);
/// assert_eq!(vec3.z, 3);
/// ```
pub struct Vector3<T> {
    /// Field holding the first dimensional-value.
    pub x: T,

    /// Field holding the second dimensional-value.
    pub y: T,

    /// Field holding the third dimensional-value.
    pub z: T,
}


/// A Vector for holding four dimensional-values.
/// 
/// # Example
/// ```rust
/// use fixed_vectors::Vector4;
/// 
/// let vec4 = Vector4::new(1, 2, 3, 4);
/// assert_eq!(vec4.x, 1);
/// assert_eq!(vec4.y, 2);
/// assert_eq!(vec4.z, 3);
/// assert_eq!(vec4.w, 4);
/// ```
pub struct Vector4<T> {
    /// Field holding the first dimensional-value.
    pub x: T,

    /// Field holding the second dimensional-value.
    pub y: T,

    /// Field holding the third dimensional-value.
    pub z: T,

    /// Field holding the fourth dimensional-value.
    pub w: T,
}


macros::impl_vector!(Vector2 { x, y }, 2);
macros::impl_vector!(Vector3 { x, y, z }, 3);
macros::impl_vector!(Vector4 { x, y, z, w}, 4);