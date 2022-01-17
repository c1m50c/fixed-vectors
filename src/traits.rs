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
    /// Raises all numbers within the [`Vector`] to the specified power.
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
    /// Converts all numbers within the [`Vector`] to the largest integer less than or equal to the value.
    /// 
    /// ## Example
    /// ```rust
    /// let vector = Vector2::new(4.25, 5.9).floor();
    /// assert_eq!(vector, Vector2::new(4.0, 5.0));
    /// ```
    fn floor(self) -> Self;

    /// Converts all numbers within the [`Vector`] to the largest integer greater than or equal to the value.
    /// 
    /// ## Example
    /// ```rust
    /// let vector = Vector2::new(4.25, 5.9).ceil();
    /// assert_eq!(vector, Vector2::new(5.0, 6.0));
    /// ```
    fn ceil(self) -> Self;

    /// Converts all numbers within the [`Vector`] to the nearest integer.
    /// 
    /// ## Example
    /// ```rust
    /// let vector = Vector2::new(4.25, 5.9).round();
    /// assert_eq!(vector, Vector2::new(4.0, 6.0));
    /// ```
    fn round(self) -> Self;

    /// Converts all numbers within the [`Vector`] to their absolute value.
    /// 
    /// ## Example
    /// ```rust
    /// let vector = Vector4::new(-3.0, 4.0, 5.3, -9.87).abs();
    /// assert_eq!(vector, Vector4::new(3.0, 4.0, 5.3, 9.87));
    /// ```
    fn abs(self) -> Self;

    /// Raises all numbers within the [`Vector`] to an integer power.
    /// 
    /// ## Example
    /// ```rust
    /// let vector = Vector2::new(2.0, 4.0).powi(2);
    /// assert_eq!(vector, Vector2::new(4, 16));
    /// ```
    fn powi(self, n: i32) -> Self;

    /// Raises all numbers within the [`Vector`] to a floating point power.
    /// 
    /// ## Example
    /// ```rust
    /// let vector = Vector2::new(2.0, 4.0).powf(2.0);
    /// assert_eq!(vector, Vector2::new(4.0, 16.0));
    /// ```
    fn powf(self, n: T) -> Self;
}


/// Trait for structs that represent a [`Vector`] and that can be converted into tuples.
pub trait TuplableVector<T, const LEN: usize>: Vector<T, LEN> {
    type Output;
    
    /// Converts the [`Vector`] into a tuple representing its values.
    /// 
    /// ## Example:
    /// ```rust
    /// let tuple = Vector2::new(1, 2).to_tuple();
    /// assert_eq!(tuple, (1, 2));
    /// ```
    fn to_tuple(self) -> Self::Output;
}