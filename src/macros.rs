macro_rules! impl_vector {
    ($struct: ident { $($field: ident), + }, $len: expr) => {
        impl<T> $struct<T> {
            /// Constructs a new Vector with the specified values for each field.
            /// 
            /// # Example
            /// ```rust
            /// use fixed_vectors::Vector2;
            /// 
            /// let vec = Vector2::new(1, 2);
            /// assert_eq!(vec.x, 1);
            /// assert_eq!(vec.y, 2);
            /// ```
            pub const fn new($($field: T), +) -> Self {
                return Self {
                    $($field: $field), +
                }
            }

            /// Returns the number of fields within the Vector as a [`usize`].
            /// 
            /// # Example
            /// ```rust
            /// use fixed_vectors::Vector2;
            /// 
            /// let len = Vector2::new(1, 2).len();
            /// assert_eq!(len, 2);
            /// ```
            pub fn len(&self) -> usize {
                return $len;
            }

            /// Connsumes the given Vector transforming it into an array.
            /// 
            /// # Example
            /// ```rust
            /// use fixed_vectors::Vector2;
            /// 
            /// let arr = Vector2::new(1, 2).to_array();
            /// assert_eq!(arr, [1, 2]);
            /// ```
            pub fn to_array(self) -> [T; $len] {
                return [ $(self.$field), + ]
            }

            /// Consumes the given Vector transforming it into a [`Vec`].
            /// 
            /// # Example
            /// ```rust
            /// use fixed_vectors::Vector2;
            /// 
            /// let vec = Vector2::new(1, 2).to_vec();
            /// assert_eq!(vec, vec![1, 2]);
            /// ```
            pub fn to_vec(self) -> std::vec::Vec<T> {
                let mut vec = std::vec::Vec::with_capacity(self.len());
                $( vec.push(self.$field); ) +
                return vec;
            }
        }
    };
}

pub(crate) use impl_vector;