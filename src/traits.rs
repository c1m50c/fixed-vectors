//! Traits for [`Vector`]s that implement very handy and convient functions.
//! [`FloatingPointVector`] for example implements `normalized()`, which is used heavily in game development.
//! 
//! ## Traits
//! ```rust
//! pub trait Vector<T, const LEN: usize>: IntoIterator { .. } // For representing a basic [`Vector`].
//! pub trait IntegerVector<T, const LEN: usize>: Vector { .. } // For representing a [`Vector`] with Integers as its generic.
//! pub trait FloatingPointVector<T, const LEN: usize>: Vector { .. } // For representing a [`Vector`] with Floating-point Numbers as its generic.
//! pub trait TuplableVector<T, const LEN: usize>: Vector { .. } // For representing a [`Vector`] that can be converted into a tuple.
//! ```


/// Trait for structs that represent a [`Vector`], will be implemented by default when using the [`impl_vector`] Macro.
pub trait Vector<T, const LEN: usize>: IntoIterator {
    /// Returns the name of the [`Vector`] struct.
    /// 
    /// ## Example
    /// ```rust
    /// let vector = Vector4::new(0, 0, 0, 0);
    /// assert_eq!(vector.name(), "Vector4");
    /// ```
    fn name(&self) -> &'static str;

    /// Returns the size of the [`Vector`] struct in bytes.
    /// 
    /// ## Example
    /// ```rust
    /// let vector = Vector2::<u16>::new(0, 0);
    /// assert_eq!(vector.size(), 4);
    /// ```
    fn size(&self) -> usize {
        return core::mem::size_of::<T>() * LEN;
    }

    /// Returns the length of the [`Vector`] struct, length is equal to the number of fields within the struct.
    /// 
    /// ## Example
    /// ```rust
    /// let vec2 = Vector2::new(0, 0);
    /// let vec3 = Vector3::new(0, 0, 0);
    /// let vec4 = Vector4::new(0, 0, 0, 0);
    /// 
    /// assert_eq!(vec2.len(), 2);
    /// assert_eq!(vec3.len(), 3);
    /// assert_eq!(vec4.len(), 4);
    /// ```
    fn len(&self) -> usize {
        return LEN;
    }

    /// Converts the given [`Vector`] into an array coresponding to the size of the [`Vector`].
    /// 
    /// ## Example
    /// ```rust
    /// let vector = Vector3::new(1, 2, 3);
    /// assert_eq!(vector.to_array(), [1, 2, 3]);
    /// ```
    fn to_array(self) -> [T; LEN];

    /// Converts the given [`Vector`] into a [`Vec`] coresponding to the size of the [`Vector`].
    /// 
    /// ## Example
    /// ```rust
    /// let vector = Vector3::new(1, 2, 3);
    /// assert_eq!(vector.to_vec(), vec![1, 2, 3]);
    /// ```
    fn to_vec(self) -> std::vec::Vec<T>;
}


/// Trait for structs that represent a [`Vector`] that contains primitive integer data types.
pub trait IntegerVector<T: num_traits::PrimInt, const LEN: usize>: Vector<T, LEN> {
    /// Raises all numbers within the [`IntegerVector`] to the specified power.
    /// 
    /// ## Example
    /// ```rust
    /// let vector = Vector3::new(2, 4, 6).pow(2);
    /// assert_eq!(vector, Vector3::new(4, 16, 36));
    /// ```
    fn pow(self, n: u32) -> Self;
}


/// Trait for structs that represent a [`Vector`] that contains floating-point data types.
pub trait FloatingPointVector<T: num_traits::Float, const LEN: usize>: Vector<T, LEN> {
    /// Converts all numbers within the [`FloatingPointVector`] to zero.
    /// 
    /// ## Example
    /// ```rust
    /// let vector = Vector4::new(1.0, 2.0, 3.0, 4.0).zero();
    /// assert_eq!(vector, Vector4::new(0.0, 0.0, 0.0, 0.0));
    /// ```
    fn zero(self) -> Self;

    /// Converts all numbers within the [`FloatingPointVector`] to the largest integer less than or equal to the value.
    /// 
    /// ## Example
    /// ```rust
    /// let vector = Vector2::new(4.25, 5.9).floor();
    /// assert_eq!(vector, Vector2::new(4.0, 5.0));
    /// ```
    fn floor(self) -> Self;

    /// Converts all numbers within the [`FloatingPointVector`] to the largest integer greater than or equal to the value.
    /// 
    /// ## Example
    /// ```rust
    /// let vector = Vector2::new(4.25, 5.9).ceil();
    /// assert_eq!(vector, Vector2::new(5.0, 6.0));
    /// ```
    fn ceil(self) -> Self;

    /// Converts all numbers within the [`FloatingPointVector`] to the nearest integer.
    /// 
    /// ## Example
    /// ```rust
    /// let vector = Vector2::new(4.25, 5.9).round();
    /// assert_eq!(vector, Vector2::new(4.0, 6.0));
    /// ```
    fn round(self) -> Self;

    /// Converts all numbers within the [`FloatingPointVector`] to their absolute value.
    /// 
    /// ## Example
    /// ```rust
    /// let vector = Vector4::new(-3.0, 4.0, 5.3, -9.87).abs();
    /// assert_eq!(vector, Vector4::new(3.0, 4.0, 5.3, 9.87));
    /// ```
    fn abs(self) -> Self;

    /// Raises all numbers within the [`FloatingPointVector`] to an integer power.
    /// 
    /// ## Example
    /// ```rust
    /// let vector = Vector2::new(2.0, 4.0).powi(2);
    /// assert_eq!(vector, Vector2::new(4, 16));
    /// ```
    fn powi(self, n: i32) -> Self;

    /// Raises all numbers within the [`FloatingPointVector`] to a floating point power.
    /// 
    /// ## Example
    /// ```rust
    /// let vector = Vector2::new(2.0, 4.0).powf(2.0);
    /// assert_eq!(vector, Vector2::new(4.0, 16.0));
    /// ```
    fn powf(self, n: T) -> Self;

    /// Sets all numbers within the [`FloatingPointVector`] to their integer parts.
    /// 
    /// ## Example
    /// ```rust
    /// let vector = Vector3::new(1.5, 2.34, 3.33).trunc();
    /// assert_eq!(vector, Vector3::new(1.0, 2.0, 3.0));
    /// ```
    fn trunc(self) -> Self;

    /// Sets all numbers within the [`FloatingPointVector`] to their fractional parts.
    /// 
    /// ## Example
    /// ```rust
    /// let vector = Vector3::new(1.5, 2.34, 3.33).fract();
    /// assert_eq!(vector, Vector3::new(0.5, 0.34, 0.33));
    /// ```
    fn fract(self) -> Self;

    /// Sets all numbers within the [`FloatingPointVector`] to their square-root.
    /// 
    /// ## Example
    /// ```rust
    /// let vector = Vector2::new(64.0, 25.0);
    /// assert_eq!(vector, Vector2::new(8.0, 5.0));
    /// ```
    fn sqrt(self) -> Self;

    /// Normalizes the [`FloatingPointVector`].
    /// 
    /// ## Example
    /// ```rust
    /// let vector = Vector2::new(14.3, 7.9).normalized();
    /// assert_eq!(vector, Vector2::new(0.8753097187762677, 0.48356271177150456));
    /// ```
    fn normalized(self) -> Self;

    /// Linearly interpolates between two [`FloatingPointVector`]s by a normalized `weight`.
    /// 
    /// ## Example
    /// ```rust
    /// let vector = Vector2::new(1.0, 2.0).lerp(Vector2::new(2.0, 3.0), 1.0);
    /// assert_eq!(vector, Vector2::new(2.0, 3.0));
    /// ```
    fn lerp(self, to: Self, weight: T) -> Self;

    /// Returns the dot product of two [`FloatingPointVector`]s,
    /// this can be used to compare the angle between two [`FloatingPointVector`]s.
    /// 
    /// ## Example
    /// ```rust
    /// let dot = Vector2::new(1.0, 2.0).dot(Vector2::new(2.0, 4.0));
    /// assert_eq!(dot, 10.0);
    /// ```
    fn dot(self, b: Self) -> T;

    /// Returns a normalized [`FloatingPointVector`] pointing from it to `to`.
    /// 
    /// ## Example
    /// ```rust
    /// let from = Vector2::new(1.0, 2.0);
    /// let to = Vector2::new(5.0, 6.0);
    ///
    /// let direction = from.direction(to);
    ///
    /// assert_eq!(direction, Vector2::new(0.7071067811865475, 0.7071067811865475));
    /// ```
    fn direction(self, to: Self) -> Self
    where
        Self: Sized + core::ops::Sub<Output = Self>
    {
        return (to - self).normalized();
    }
}


/// Trait for structs that represent a [`Vector`] and that can be converted into tuples.
/// 
/// ## Example
/// ```rust
/// pub struct Vector1<T> {
///     pub x: T,
/// }
/// 
/// // Implement [`Vector`] Trait for `Vector1`
/// impl_vector!(Vector1 { x }, 1);
/// 
/// impl<T> TuplableVector<T, { Vector1::<()>::LEN }> for Vector1<T> {
///     type Output = (T);
///     
///     fn to_tuple(self) -> Self::Output {
///         return (self.x);
///     }
/// }
/// ```
pub trait TuplableVector<T, const LEN: usize>: Vector<T, LEN> {
    type Output;
    
    /// Converts the [`TuplableVector`] into a tuple representing its values.
    /// 
    /// ## Example:
    /// ```rust
    /// let tuple = Vector2::new(1, 2).to_tuple();
    /// assert_eq!(tuple, (1, 2));
    /// ```
    fn to_tuple(self) -> Self::Output;
}