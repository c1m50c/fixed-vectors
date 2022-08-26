//! Library containing a collection of fixed-size Vectors.
//! These Vectors are useful for use in positional, and dimensional values.
//! There are three built-in Vectors to choose from, the built-in Vectors are the structs
//! [`Vector2`], [`Vector3`], and [`Vector4`].
//! 
//! # Example
//! ```rust
//! use fixed_vectors::Vector3;
//! 
//! let vec = Vector3::new(1, 2, 3);
//! let sum = vec.into_iter().sum::<i32>();
//! assert_eq!(sum, 6);
//! ```

#[cfg(test)]
mod tests;

#[cfg(feature = "macros")]
pub mod macros;

#[cfg(not(feature = "macros"))]
mod macros;

/// The type returned in [`Result`]s created by Vector functions.
/// 
/// # Example
/// ```rust
/// use fixed_vectors::{Vector2, VectorError};
/// use std::convert::TryFrom;
/// 
/// // This returns an `Err` because the given `Vec` is not as big as a `Vector2`
/// let try_from_vec = Vector2::try_from(vec![1]);
/// 
/// if let Err(error) = try_from_vec {
///     assert_eq!(error, VectorError::CannotConvertFromImproperlySizedCollection);
/// }
/// ```
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum VectorError {
    /// Returned when there's not enough information to give a more verbose [`VectorError`].
    GenericError,

    /// Given when converting a Dynamically Sized Collection into a Vector,
    /// only when the Collection's size is less than the Vector's.
    /// 
    /// # Example
    /// ```rust
    /// use fixed_vectors::{Vector3, VectorError};
    /// use std::convert::TryFrom;
    /// 
    /// let vec = vec![1, 2];
    /// let res = Vector3::try_from(vec);
    /// assert_eq!(res.err().unwrap(), VectorError::CannotConvertFromImproperlySizedCollection);
    /// ```
    CannotConvertFromImproperlySizedCollection
}


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

macros::impl_vector!(Vector2 { x, y } -> (T, T), 2);
macros::impl_vector!(Vector3 { x, y, z } -> (T, T, T), 3);
macros::impl_vector!(Vector4 { x, y, z, w } -> (T, T, T, T), 4);