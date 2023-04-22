#![no_std]

mod macros;

#[cfg(test)]
mod tests;


/// Vector for holding two-dimensional values.
/// 
/// # Example
/// 
/// ```
/// use fixed_vectors::Vector2;
/// 
/// let mut vec2 = Vector2::new(1, 2);
/// vec2 += Vector2::new(1, 2);
/// 
/// assert_eq!(vec2.x, 2);
/// assert_eq!(vec2.y, 4);
/// ```
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}


/// Vector for holding three-dimensional values.
/// 
/// # Example
/// 
/// ```
/// use fixed_vectors::Vector3;
/// 
/// let mut vec3 = Vector3::new(1, 2, 3);
/// vec3 += Vector3::new(1, 2, 3);
/// 
/// assert_eq!(vec3.x, 2);
/// assert_eq!(vec3.y, 4);
/// assert_eq!(vec3.z, 6);
/// ```
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}


/// Vector for holding four-dimensional values.
/// 
/// # Example
/// 
/// ```
/// use fixed_vectors::Vector4;
/// 
/// let mut vec4 = Vector4::new(1, 2, 3, 4);
/// vec4 += Vector4::new(1, 2, 3, 4);
/// 
/// assert_eq!(vec4.x, 2);
/// assert_eq!(vec4.y, 4);
/// assert_eq!(vec4.z, 6);
/// assert_eq!(vec4.w, 8);
/// ```
pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}


impl_vector!(Vector2 { x, y }, 2);
impl_vector!(Vector3 { x, y, z }, 3);
impl_vector!(Vector4 { x, y, z, w }, 4);