/// Macro used in implementing all the floating-point methods for Vectors.
/// 
/// # Example
/// ```
/// use fixed_vectors::impl_vector;
/// 
/// struct Vector2<T> {
///     x: T,
///     y: T,
/// }
/// 
/// impl_vector!(Vector2 { x, y } -> (T, T), 2);
/// let vec = Vector2::new(1.5, 2.5);
/// 
/// assert_eq!(vec.floor(), Vector2::new(1.0, 2.0));
/// ```
#[macro_export]
macro_rules! impl_floating_point_operations {
    ($struct: ident { $($field: ident), + }, $len: expr) => {
        #[allow(dead_code)]
        impl<T: num_traits::Float> $struct<T> {
            /// Converts all numbers within the Vector to zero.
            /// 
            /// # Example
            /// ```rust
            /// use fixed_vectors::Vector2;
            /// 
            /// let vector = Vector2::new(1.0, 2.0).zero();
            /// assert_eq!(vector, Vector2::new(0.0, 0.0));
            /// ```
            pub fn zero(self) -> Self {
                /*
                    TODO: Look into this later?
                    NOTE: 🤔 I wonder what's more performant:
                    - Mapping the given Vector with the required functions.
                    - Constructing a new Vector calling the required function on each field.
                */
                return self.map(|_| T::zero());
            }

            /// Converts all numbers within the Vector to largest integer
            /// less than or equal to the value.
            /// 
            /// # Example
            /// ```rust
            /// use fixed_vectors::Vector2;
            /// 
            /// let vector = Vector2::new(1.25, 5.9).floor();
            /// assert_eq!(vector, Vector2::new(1.0, 5.0));
            /// ```
            pub fn floor(self) -> Self {
                return self.map(|f| f.floor());
            }

            /// Converts all numbers within the Vector to largest integer
            /// greater than or equal to the value.
            /// 
            /// # Example
            /// ```rust
            /// use fixed_vectors::Vector2;
            /// 
            /// let vector = Vector2::new(1.25, 5.9).ceil();
            /// assert_eq!(vector, Vector2::new(2.0, 6.0));
            /// ```
            pub fn ceil(self) -> Self {
                return self.map(|f| f.ceil());
            }

            /// Converts all numbers within the Vector to the nearest integer.
            /// 
            /// # Example
            /// ```rust
            /// use fixed_vectors::Vector2;
            /// 
            /// let vector = Vector2::new(1.25, 5.9).round();
            /// assert_eq!(vector, Vector2::new(1.0, 6.0));
            /// ```
            pub fn round(self) -> Self {
                return self.map(|f| f.round());
            }

            /// Converts all numbers within the Vector to their absolute value.
            /// 
            /// # Example
            /// ```rust
            /// use fixed_vectors::Vector2;
            /// 
            /// let vector = Vector2::new(-1.0, 1.0).abs();
            /// assert_eq!(vector, Vector2::new(1.0, 1.0));
            /// ```
            pub fn abs(self) -> Self {
                return self.map(|f| f.abs());
            }

            /// Converts all numbers within the Vector to their integer parts.
            /// 
            /// # Example
            /// ```rust
            /// use fixed_vectors::Vector2;
            /// 
            /// let vector = Vector2::new(1.337, 2.5).trunc();
            /// assert_eq!(vector, Vector2::new(1.0, 2.0));
            /// ```
            pub fn trunc(self) -> Self {
                return self.map(|f| f.trunc());
            }

            /// Converts all numbers within the Vector to their fractional parts.
            /// 
            /// # Example
            /// ```rust
            /// use fixed_vectors::Vector2;
            /// 
            /// let vector = Vector2::new(1.337, 2.5).fract();
            /// assert!(vector.x <= 0.337);
            /// assert!(vector.y <= 0.5);
            /// ```
            pub fn fract(self) -> Self {
                return self.map(|f| f.fract());
            }

            /// Converts all numbers within the Vector to their square-root.
            /// 
            /// # Example
            /// ```rust
            /// use fixed_vectors::Vector2;
            /// 
            /// let vector = Vector2::new(64.0, 25.0).sqrt();
            /// assert_eq!(vector, Vector2::new(8.0, 5.0));
            /// ```
            pub fn sqrt(self) -> Self {
                return self.map(|f| f.sqrt());
            }

            /// Raises all numbers within the Vector to an integer power.
            /// 
            /// # Example
            /// ```rust
            /// use fixed_vectors::Vector2;
            /// 
            /// let vector = Vector2::new(2.0, 4.0).powi(2);
            /// assert_eq!(vector, Vector2::new(4.0, 16.0));
            /// ```
            pub fn powi(self, n: i32) -> Self {
                return self.map(|f| f.powi(n));
            }

            /// Raises all numbers within the Vector to a floating-point power.
            /// 
            /// # Example
            /// ```rust
            /// use fixed_vectors::Vector2;
            /// 
            /// let vector = Vector2::new(2.0, 4.0).powf(2.0);
            /// assert_eq!(vector, Vector2::new(4.0, 16.0));
            /// ```
            pub fn powf(self, n: T) -> Self {
                return self.map(|f| f.powf(n));
            }

            /// Linearly interpolates between two Vectors by a normalized `weight`.
            /// 
            /// # Example
            /// ```rust
            /// use fixed_vectors::Vector2;
            /// 
            /// let vector = Vector2::new(1.0, 2.0).lerp(Vector2::new(2.0, 3.0), 1.0);
            /// assert_eq!(vector, Vector2::new(2.0, 3.0));
            /// ```
            pub fn lerp(self, to: Self, weight: T) -> Self {
                return Self {
                    $( $field: self.$field + (weight * (to.$field - self.$field)) ), +
                };
            }

            /// Normalizes the Vector.
            /// 
            /// # Example
            /// ```rust
            /// use fixed_vectors::Vector2;
            /// 
            /// let vector = Vector2::new(14.3, 7.9).normalized();
            /// assert!(vector.x < 1.0);
            /// assert!(vector.y < 1.0);
            /// ```
            pub fn normalized(self) -> Self {
                let mut iter = self.clone().into_iter()
                    .map(|f| f * f);

                let start = iter.next()
                    .expect("Cannot normalize a Vector that contains no fields.");
                let mut magnitude = iter.fold(start, |acc, f| acc * f);

                if magnitude == T::zero() { return self.zero(); }
                magnitude = magnitude.sqrt();

                return Self {
                    $( $field: self.$field / magnitude ), +
                };
            }

            /// Returns the dot product of two Vectors,
            /// this can be used to compare the angle between two Vectors.
            /// 
            /// # Example
            /// ```rust
            /// use fixed_vectors::Vector2;
            /// 
            /// let dot = Vector2::new(1.0, 2.0).dot(Vector2::new(2.0, 4.0));
            /// assert_eq!(dot, 16.0);
            /// ```
            pub fn dot(self, b: Self) -> T {
                let mut iter = self.into_iter()
                    .zip(b.into_iter())
                    .map(|(a, b)| a * b);

                let start = iter.next()
                    .expect("Cannot retrieve the dot product of a Vector that contains no fields.");
                return iter.fold(start, |acc, f| acc * f);
            }

            /// Returns the squared magnitude of the Vector.
            /// This will always run faster than [`length`] and should be prefered over it if applicable.
            /// 
            /// # Example
            /// ```rust
            /// use fixed_vectors::Vector3;
            /// 
            /// let lsq = Vector3::new(3.33, 2.04, 1.337).length_squared();
            /// assert!(lsq >= 17.0);
            /// ```
            pub fn length_squared(self) -> T {
                return self.into_iter()
                    .map(|f| f * f)
                    .fold(T::zero(), |acc, f| acc + f);
            }

            /// Returns the magnitude of the Vector.
            /// 
            /// # Example
            /// ```rust
            /// use fixed_vectors::Vector3;
            /// 
            /// let len = Vector3::new(1.5, 2.0, 3.33).length();
            /// assert!(len < 4.2);
            /// ```
            pub fn length(self) -> T {
                return self.length_squared().sqrt();
            }

            /// Returns a normalized Vector pointing from it to `to`.
            /// 
            /// # Example
            /// ```rust
            /// use fixed_vectors::Vector2;
            /// 
            /// let from = Vector2::new(1.0, 2.0);
            /// let to = Vector2::new(5.0, 6.0);
            /// 
            /// let dir = from.direction(to);
            /// assert!(dir.x < 1.0);
            /// assert!(dir.y < 1.0);
            /// ```
            pub fn direction(self, to: Self) -> Self {
                return (to - self).normalized();
            }
        }
    }
}