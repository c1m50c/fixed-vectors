mod operators;
mod floating;

/// Macro used in implementing all the methods for Vectors.
/// 
/// # Example
/// ```rust
/// use fixed_vectors::macros::impl_vector;
/// 
/// struct Vector2<T> {
///     x: T,
///     y: T,
/// }
/// 
/// impl_vector!(Vector2 { x, y }, 2);
/// let vec = Vector2::new(1, 2);
/// 
/// assert_eq!(vec.len(), 2);
/// assert_eq!(vec.x, 1);
/// assert_eq!(vec.y, 2);
/// ```
macro_rules! impl_vector {
    ($struct: ident { $($field: ident), + } -> $tuple_type: tt, $len: expr) => {
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
            pub const fn new($( $field: T ), +) -> Self {
                return Self {
                    $( $field: $field ), +
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
            pub const fn len(&self) -> usize {
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

            /// Consumes the given Vector transforming it into a tuple.
            /// 
            /// # Example
            /// ```rust
            /// use fixed_vectors::Vector2;
            /// 
            /// let tuple = Vector2::new(1, 2).to_tuple();
            /// assert_eq!(tuple, (1, 2));
            /// ```
            pub fn to_tuple(self) -> $tuple_type {
                return ( $( self.$field ), + );
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
                let mut vec = std::vec::Vec::with_capacity($len);
                $( vec.push(self.$field); ) +
                return vec;
            }

            /// Takes a closure and creates a Vector that called the given closure on each field.
            /// 
            /// # Example
            /// ```rust
            /// use fixed_vectors::Vector2;
            /// 
            /// let vec = Vector2::new(1, 2)
            ///     .map(|i| i as f64);
            /// assert_eq!(vec, Vector2::new(1.0, 2.0));
            /// ```
            pub fn map<F, R>(self, mut f: F) -> $struct<R>
            where
                F: FnMut(T) -> R
            {
                return $struct {
                    $( $field: f(self.$field) ), +
                };
            }
        }

        impl<T: Default> Default for $struct<T> {
            fn default() -> Self {
                return Self {
                    $( $field: T::default() ), +
                };
            }
        }

        impl<T: Clone> Clone for $struct<T> {
            fn clone(&self) -> Self {
                return Self {
                    $( $field: self.$field.clone() ), +
                };
            }
        }

        impl<T> IntoIterator for $struct<T> {
            type IntoIter = std::array::IntoIter<T, $len>;
            type Item = T;
            
            fn into_iter(self) -> Self::IntoIter {
                return std::iter::IntoIterator::into_iter(self.to_array());
            }
        }

        impl<T> From<[T; $len]> for $struct<T> {
            fn from(f: [T; $len]) -> Self {
                let mut iter = f.into_iter();
                return Self {
                    $( $field: iter.next().unwrap() ), +
                }
            }
        }

        impl<T> From<$tuple_type> for $struct<T> {
            fn from(f: $tuple_type) -> Self {
                return match f {
                    ($( $field ), +) => {
                        Self { $( $field ), + }
                    },
                };
            }
        }

        impl<T> TryFrom<std::vec::Vec<T>> for $struct<T> {
            type Error = $crate::VectorError;
            fn try_from(f: std::vec::Vec<T>) -> Result<Self, Self::Error> {
                if f.len() < $len {
                    return Err(Self::Error::CannotConvertFromImproperlySizedCollection);
                }

                let mut iter = f.into_iter();
                return Ok(Self {
                    $( $field: iter.next().ok_or(Self::Error::GenericError)? ), +
                })
            }
        }

        impl<T: core::fmt::Debug> core::fmt::Debug for $struct<T> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                return f.debug_struct(stringify!($struct))
                    $( .field(stringify!($field), &self.$field) ) +
                    .finish();
            }
        }

        impl<T: core::fmt::Display> core::fmt::Display for $struct<T> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                if $len == 0 {
                    return write!(f, "()");
                }

                let mut result = String::from("(");
                $( result.push_str(format!("{}, ", &self.$field).as_str()); ) +
                return write!(f, "{}", result.strip_suffix(", ").unwrap().to_string() + ")");
            }
        }

        impl<T: core::hash::Hash> core::hash::Hash for $struct<T> {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                $( self.$field.hash(state); ) +
            }
        }

        $crate::macros::impl_operators!($struct { $($field), + }, $len);
        $crate::macros::impl_floating_point_operations!($struct { $($field), + }, $len);
    };
}

pub(crate) use impl_vector;
pub(crate) use operators::impl_operators;
pub(crate) use floating::impl_floating_point_operations;
