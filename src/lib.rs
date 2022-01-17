use core::iter::{Iterator, IntoIterator, FusedIterator, DoubleEndedIterator, ExactSizeIterator};
use core::cmp::PartialEq;

#[cfg(test)]
mod tests;


/// Struct for the [`IntoIter`] trait used by [`Vector`]s.
/// Iterator works based off going from the `x` field to the remaining fields.
/// 
/// ## Example
/// ```rust
/// let mut iter = Vector3::new("Vector", "3", "Iterator").into_iter();
/// assert_eq!(iter.next(), Some("Vector"));
/// assert_eq!(iter.next(), Some("3"));
/// assert_eq!(iter.next(), Some("Iterator"));
/// assert_eq!(iter.next(), None);
/// ```
/// 
/// ## Fields
/// ```rust
/// pub vec: std::vec::Vec<T>
/// ```
pub struct IntoIter<T> {
    pub vec: std::vec::Vec<T>,
}


impl<T> Iterator for IntoIter<T> {
    type Item = T;
    
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.vec.len() == 0 { return None; }
        return Some(self.vec.remove(0));
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        return self.vec.iter().size_hint();
    }
}


impl<T> DoubleEndedIterator for IntoIter<T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        return self.vec.pop();
    }
}


impl<T> ExactSizeIterator for IntoIter<T> {  }
impl<T> FusedIterator for IntoIter<T> {  }


/// Macros for implementing [`Vector`] functions & constants in `struct`s.
/// 
/// ## Example
/// ```rust
/// struct Vector5<T> {
///     x: T,
///     y: T,
///     z: T,
///     w: T,
///     v: T,
/// }
/// 
/// impl_vector!(Vector5 { x, y, z, w, v }, 5);
/// 
/// // Now the struct has generic Vector methods.
/// let vector = Vector5::new(1, 2, 3, 4, 5);
/// assert_eq!(format!("{}", vector), "(1, 2, 3, 4, 5)");
/// 
/// assert_eq!(Vector5::NAME, "Vector5");
/// assert_eq!(Vector5::LEN, 5);
/// ```
/// 
/// ## Parameters
/// ```rust
/// $Vector: ident // Name of the Vector Struct
/// { $($field: ident), + } // Fields within the Vector Struct
/// $len: expr // The amount of fields within the Vector Struct
/// ```
/// 
/// ## `Impl` Constants
/// ```rust
/// pub const NAME: &'static str = stringify!($Vector)
/// pub const SIZE: usize = core::mem::size_of::<T>() * $len
/// pub const LEN: usize = $len
/// ```
/// 
/// ## `Impl` Functions
/// ```rust
/// pub const fn new() -> Vector<T>
/// pub fn to_array(self) -> [T; $len]
/// pub fn to_vec(self) -> Vec<T>
/// ```
/// 
/// To see more implemented traits & functions, see the source code.
#[macro_export]
macro_rules! impl_vector {
    ($Vector: ident { $($field: ident), + }, $len: expr) => {
        impl<T> $Vector<T> {
            /// Name of the [`Vector`] Struct as a `static str`.
            pub const NAME: &'static str = stringify!($Vector);

            /// Size of the [`Vector`] in Bytes, calculated based of `size_of::<T>()` * `Vector::LEN`.
            pub const SIZE: usize = core::mem::size_of::<T>() * $len;

            /// Length of the [`Vector`] Struct as a `usize`.
            pub const LEN: usize = $len;

            /// Creates a new [`Vector`] with the specified values for the fields.
            /// 
            /// ## Example
            /// ```rust
            /// let vector = Vector2::new("Vector", "2");
            /// assert_eq!(vector.x, "Vector");
            /// assert_eq!(vector.y, "2");
            /// ```
            #[inline]
            pub const fn new($( $field: T ), +) -> $Vector<T> {
                return Self {
                    $( $field: $field ), +
                };
            }

            /// Converts the given [`Vector`] into an array coresponding to the size of the [`Vector`].
            /// 
            /// ## Example
            /// ```rust
            /// let vector = Vector3::new(1, 2, 3);
            /// assert_eq!(vector.to_array(), [1, 2, 3]);
            /// ```
            #[inline]
            pub fn to_array(self) -> [T; $len] {
                return [
                    $(self.$field), +
                ];
            }

            /// Converts the given [`Vector`] into a [`Vec`] coresponding to the size of the [`Vector`].
            /// 
            /// ## Example
            /// ```rust
            /// let vector = Vector3::new(1, 2, 3);
            /// assert_eq!(vector.to_vec(), vec![1, 2, 3]);
            /// ```
            #[inline]
            pub fn to_vec(self) -> std::vec::Vec<T> {
                let mut vec = std::vec::Vec::with_capacity(Self::LEN);
                $( vec.push(self.$field); ) +
                return vec;
            }
        }

        impl<T: PartialOrd + Copy> $Vector<T> {
            /// Returns the maximum value contained within the [`Vector`].
            /// 
            /// ## Example
            /// ```rust
            /// let vector = Vector3::new(1, 0, 2);
            /// assert_eq!(vector.max(), 2);
            /// ```
            #[inline]
            pub fn max(&self) -> T {
                let mut iter = self.into_iter();
                let mut result = iter.next().expect("Cannot retrieve max value on zero-lengthed Vector.");
        
                for v in iter {
                    if v > result { result = v; }
                }
        
                return result;
            }

            /// Returns the minimum value contained within the [`Vector`].
            /// 
            /// ## Example
            /// ```rust
            /// let vector = Vector3::new(1, 0, 2);
            /// assert_eq!(vector.min(), 0);
            /// ```
            #[inline]
            pub fn min(&self) -> T {
                let mut iter = self.into_iter();
                let mut result = iter.next().expect("Cannot retrieve min value on zero-lengthed Vector.");
        
                for v in iter {
                    if v < result { result = v; }
                }
        
                return result;
            }
        }

        impl<T: num_traits::Float> $Vector<T> {
            /// Converts all numbers within the [`Vector`] to the largest integer less than or equal to the value.
            /// 
            /// ## Example
            /// ```rust
            /// let vector = Vector2::new(4.25, 5.9).floor();
            /// assert_eq!(vector, Vector2::new(4.0, 5.0));
            /// ```
            #[inline]
            pub fn floor(self) -> Self {
                return Self {
                    $( $field: self.$field.floor() ), +
                };
            }

            /// Converts all numbers within the [`Vector`] to the largest integer greater than or equal to the value.
            /// 
            /// ## Example
            /// ```rust
            /// let vector = Vector2::new(4.25, 5.9).ceil();
            /// assert_eq!(vector, Vector2::new(5.0, 6.0));
            /// ```
            #[inline]
            pub fn ceil(self) -> Self {
                return Self {
                    $( $field: self.$field.ceil() ), +
                }
            }

            /// Converts all numbers within the [`Vector`] to the nearest integer.
            /// 
            /// ## Example
            /// ```rust
            /// let vector = Vector2::new(4.25, 5.9).round();
            /// assert_eq!(vector, Vector2::new(4.0, 6.0));
            /// ```
            #[inline]
            pub fn round(self) -> Self {
                return Self {
                    $( $field: self.$field.round() ), +
                }
            }

            /// Converts all numbers within the [`Vector`] to their absolute value.
            /// 
            /// ## Example
            /// ```rust
            /// let vector = Vector4::new(-3.0, 4.0, 5.3, -9.87).abs();
            /// assert_eq!(vector, Vector4::new(3.0, 4.0, 5.3, 9.87));
            /// ```
            #[inline]
            pub fn abs(self) -> Self {
                return Self {
                    $( $field: self.$field.abs() ), +
                };
            }

            /// Raises all numbers within the [`Vector`] to an integer power.
            /// 
            /// ## Example
            /// ```rust
            /// let vector = Vector2::new(2.0, 4.0).powi(2);
            /// assert_eq!(vector, Vector2::new(4, 16));
            /// ```
            #[inline]
            pub fn powi(self, n: i32) -> Self {
                return Self {
                    $( $field: self.$field.powi(n) ), +
                };
            }

            /// Raises all numbers within the [`Vector`] to a floating point power.
            /// 
            /// ## Example
            /// ```rust
            /// let vector = Vector2::new(2.0, 4.0).powf(2.0);
            /// assert_eq!(vector, Vector2::new(4.0, 16.0));
            /// ```
            #[inline]
            pub fn powf(self, n: T) -> Self {
                return Self {
                    $( $field: self.$field.powf(n) ), +
                };
            }
        }

        impl<T: num_traits::PrimInt> $Vector<T> {
            /// Raises all numbers within the [`Vector`] to the specified power.
            /// 
            /// ## Example
            /// ```rust
            /// let vector = Vector3::new(2, 4, 6).pow(2);
            /// assert_eq!(vector, Vector3::new(4, 16, 36));
            /// ```
            #[inline]
            pub fn pow(self, exp: u32) -> Self {
                return Self {
                    $( $field: self.$field.pow(exp) ), +
                };
            }
        }

        impl<T: Default> Default for $Vector<T> {
            #[inline]
            fn default() -> $Vector<T> {
                return Self {
                    $( $field: T::default() ), +
                };
            }
        }

        impl<T: Copy> Copy for $Vector<T> {  }

        impl<T: Copy> Clone for $Vector<T> {
            #[inline]
            fn clone(&self) -> $Vector<T> {
                return *self;
            }
        }

        impl<T: core::ops::Add<Output = T>> core::ops::Add for $Vector<T> {
            type Output = Self;

            #[inline]
            fn add(self, other: Self) -> Self::Output {
                return Self {
                    $( $field: self.$field + other.$field ), +
                };
            }
        }

        impl<T: core::ops::Add<Output = T> + Copy> core::ops::AddAssign for $Vector<T> {
            #[inline]
            fn add_assign(&mut self, other: Self) {
                *self = Self {
                    $( $field: self.$field + other.$field ), +
                };
            }
        }

        impl<T: core::ops::Sub<Output = T>> core::ops::Sub for $Vector<T> {
            type Output = Self;

            #[inline]
            fn sub(self, other: Self) -> Self::Output {
                return Self {
                    $( $field: self.$field - other.$field ), +
                };
            }
        }

        impl<T: core::ops::Sub<Output = T> + Copy> core::ops::SubAssign for $Vector<T> {
            #[inline]
            fn sub_assign(&mut self, other: Self) {
                *self = Self {
                    $( $field: self.$field - other.$field ), +
                };
            }
        }

        impl<T: core::ops::Mul<Output = T>> core::ops::Mul for $Vector<T> {
            type Output = Self;

            #[inline]
            fn mul(self, other: Self) -> Self::Output {
                return Self {
                    $( $field: self.$field * other.$field ), +
                };
            }
        }

        impl<T: core::ops::Mul<Output = T> + Copy> core::ops::MulAssign for $Vector<T> {
            #[inline]
            fn mul_assign(&mut self, other: Self) {
                *self = Self {
                    $( $field: self.$field * other.$field ), +
                };
            }
        }

        impl<T: core::ops::Div<Output = T>> core::ops::Div for $Vector<T> {
            type Output = Self;

            #[inline]
            fn div(self, other: Self) -> Self::Output {
                return Self {
                    $( $field: self.$field / other.$field ), +
                };
            }
        }

        impl<T: core::ops::Div<Output = T> + Copy> core::ops::DivAssign for $Vector<T> {
            #[inline]
            fn div_assign(&mut self, other: Self) {
                *self = Self {
                    $( $field: self.$field / other.$field ), +
                };
            }
        }

        impl<T: core::ops::Rem<Output = T>> core::ops::Rem for $Vector<T> {
            type Output = Self;

            #[inline]
            fn rem(self, other: Self) -> Self::Output {
                return Self {
                    $( $field: self.$field % other.$field ), +
                };
            }
        }

        impl<T: core::ops::Rem<Output = T> + Copy> core::ops::RemAssign for $Vector<T> {
            #[inline]
            fn rem_assign(&mut self, other: Self) {
                *self = Self {
                    $( $field: self.$field % other.$field ), +
                };
            }
        }

        impl<T: core::ops::Neg<Output = T>> core::ops::Neg for $Vector<T> {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self::Output {
                return Self {
                    $( $field: -self.$field ), +
                };
            }
        }

        impl<T: PartialEq> PartialEq for $Vector<T> {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                return $( self.$field == other.$field ) && +
            }
        }

        impl<T: Eq> Eq for $Vector<T> {  }

        impl<T: core::hash::Hash> core::hash::Hash for $Vector<T> {
            #[inline]
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                $( self.$field.hash(state); ) +
            }
        }

        impl<T: core::fmt::Debug> core::fmt::Debug for $Vector<T> {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                return f.debug_struct(stringify!($Vector))
                    $( .field(stringify!($field), &self.$field) ) +
                    .finish();
            }
        }

        impl<T: core::fmt::Display> core::fmt::Display for $Vector<T> {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                // SAFETY: Provided safety for the unwrapping in return.
                if Self::LEN == 0 { return write!(f, "()"); }
        
                let mut result = String::from("(");
                $( result.push_str(format!("{}, ", &self.$field).as_str()); ) +
                
                return write!(f, "{}", result.strip_suffix(", ").unwrap().to_string() + ")");
            }
        }

        impl<T> IntoIterator for $Vector<T> {
            type Item = T;
            type IntoIter = $crate::IntoIter<T>;
        
            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                return Self::IntoIter { vec: self.to_vec() };
            }
        }

        impl<T: Copy> From<[T; $len]> for $Vector<T> {
            #[inline]
            fn from(arr: [T; $len]) -> Self {
                let mut iter = arr.iter();
                return Self {
                    $( $field: *iter.next().unwrap() ), +
                };
            }
        }

        /*
            SAFETY: Checks the [`Vec`]'s length to ensure it is greater than or equal to the [`Vector`]'s length
        */
        impl<T: Copy> From<std::vec::Vec<T>> for $Vector<T> {
            #[inline]
            fn from(vec: std::vec::Vec<T>) -> Self {
                assert!(vec.len() >= $len, "Vec's length is less than the Vector's length.");
                let mut iter = vec.iter();
                return Self {
                    $( $field: *iter.next().unwrap() ), +
                };
            }
        }
    }
}


/// [`Vector`] Struct for representing a two-dimensional value.
/// 
/// ## Fields
/// ```rust
/// pub x: T
/// pub y: T
/// ```
/// 
/// ## Constants
/// ```rust
/// LEN == 2
/// SIZE == size_of::<T>() * LEN
/// NAME == "Vector2"
/// ```
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}


/// [`Vector`] Struct for representing a three-dimensional value.
/// 
/// ## Fields
/// ```rust
/// pub x: T
/// pub y: T
/// pub z: T
/// ```
/// 
/// ## Constants
/// ```rust
/// LEN == 3
/// SIZE == size_of::<T>() * LEN
/// NAME == "Vector3"
/// ```
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}


/// [`Vector`] Struct for representing a four-dimensional value.
/// 
/// ## Fields
/// ```rust
/// pub x: T
/// pub y: T
/// pub z: T
/// pub w: T
/// ```
/// 
/// ## Constants
/// ```rust
/// LEN == 4
/// SIZE == size_of::<T>() * LEN
/// NAME == "Vector4"
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


impl<T> Vector2<T> {
    /// Converts the [`Vector2`] into a tuple representing its values.
    /// 
    /// ## Example:
    /// ```rust
    /// let tuple = Vector2::new(1, 2).to_tuple();
    /// assert_eq!(tuple, (1, 2));
    /// ```
    #[inline]
    pub fn to_tuple(self) -> (T, T) {
        // TODO: Find a way to do this in `impl_vector` macro, this is repetitive.
        return (self.x, self.y);
    }
}


impl<T> Vector3<T> {
    /// Converts the [`Vector3`] into a tuple representing its values.
    /// 
    /// ## Example:
    /// ```rust
    /// let tuple = Vector2::new(1, 2).to_tuple();
    /// assert_eq!(tuple, (1, 2));
    /// ```
    #[inline]
    pub fn to_tuple(self) -> (T, T, T) {
        // TODO: Find a way to do this in `impl_vector` macro, this is repetitive.
        return (self.x, self.y, self.z);
    }
}


impl<T> Vector4<T> {
    /// Converts the [`Vector4`] into a tuple representing its values.
    /// 
    /// ## Example:
    /// ```rust
    /// let tuple = Vector2::new(1, 2).to_tuple();
    /// assert_eq!(tuple, (1, 2));
    /// ```
    #[inline]
    pub fn to_tuple(self) -> (T, T, T, T) {
        // TODO: Find a way to do this in `impl_vector` macro, this is repetitive.
        return (self.x, self.y, self.z, self.w);
    }
}