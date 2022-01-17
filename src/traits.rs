/// Trait for structs that represent a [`Vector`], will be implemented by default when using the [`impl_vector`] Macro.
pub trait Vector<T, const N: usize>: IntoIterator {
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
    fn size(&self) -> usize;

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
    fn len(&self) -> usize;

    /// Converts the given [`Vector`] into an array coresponding to the size of the [`Vector`].
    /// 
    /// ## Example
    /// ```rust
    /// let vector = Vector3::new(1, 2, 3);
    /// assert_eq!(vector.to_array(), [1, 2, 3]);
    /// ```
    fn to_array(self) -> [T; N];

    /// Converts the given [`Vector`] into a [`Vec`] coresponding to the size of the [`Vector`].
    /// 
    /// ## Example
    /// ```rust
    /// let vector = Vector3::new(1, 2, 3);
    /// assert_eq!(vector.to_vec(), vec![1, 2, 3]);
    /// ```
    fn to_vec(self) -> std::vec::Vec<T>;
}