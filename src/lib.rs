use core::hash::{Hash, Hasher};
use core::iter::{Iterator, IntoIterator};
use core::ops::{Add, AddAssign};
use core::ops::{Sub, SubAssign};
use core::ops::{Mul, MulAssign};
use core::ops::{Div, DivAssign};
use core::ops::{Rem, RemAssign};
use core::ops::Neg;
use core::cmp::PartialEq;
use core::mem::size_of;
use core::fmt;

use num_traits::Float;

#[cfg(test)]
mod tests;


pub struct IntoIter<T> {
    vec: std::vec::Vec<T>,
}


impl<T> Iterator for IntoIter<T> {
    type Item = T;
    
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.vec.len() == 0 { return None; }
        return Some(self.vec.remove(0));
    }
}


/// Macros for implementing various functions within Vector-like structs.
/// 
/// ## Example
/// ```rust
/// struct Vector5<T> {
///     pub x: T,
///     pub y: T,
///     pub z: T,
///     pub w: T,
///     pub v: T,
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
#[macro_export]
macro_rules! impl_vector {
    ($Vector: ident { $($field: ident), + }, $len: expr) => {
        impl<T> $Vector<T> {
            /// Name of the [`Vector`] Struct as a `static str`.
            pub const NAME: &'static str = stringify!($Vector);

            /// Size of the [`Vector`] in Bytes, calculated based of `size_of::<T>()` * `Vector::LEN`.
            pub const SIZE: usize = size_of::<T>() * $len;

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

        impl<T: Float> $Vector<T> {
            /// Converts all numbers within the [`Vector`] to the largest integer less than or equal to the value.
            /// 
            /// ## Example
            /// ```rust
            /// let vector = Vector2::new(4.25, 5.9).floor();
            /// assert_eq!(vector, Vector2::new(4.0, 5.0));
            /// ```
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
            pub fn abs(self) -> Self {
                return Self {
                    $( $field: self.$field.abs() ), +
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

        impl<T: Add<Output = T>> Add for $Vector<T> {
            type Output = Self;

            #[inline]
            fn add(self, other: Self) -> Self::Output {
                return Self {
                    $( $field: self.$field + other.$field ), +
                };
            }
        }

        impl<T: Add<Output = T> + Copy> AddAssign for $Vector<T> {
            #[inline]
            fn add_assign(&mut self, other: Self) {
                *self = Self {
                    $( $field: self.$field + other.$field ), +
                };
            }
        }

        impl<T: Sub<Output = T>> Sub for $Vector<T> {
            type Output = Self;

            #[inline]
            fn sub(self, other: Self) -> Self::Output {
                return Self {
                    $( $field: self.$field - other.$field ), +
                };
            }
        }

        impl<T: Sub<Output = T> + Copy> SubAssign for $Vector<T> {
            #[inline]
            fn sub_assign(&mut self, other: Self) {
                *self = Self {
                    $( $field: self.$field - other.$field ), +
                };
            }
        }

        impl<T: Mul<Output = T>> Mul for $Vector<T> {
            type Output = Self;

            #[inline]
            fn mul(self, other: Self) -> Self::Output {
                return Self {
                    $( $field: self.$field * other.$field ), +
                };
            }
        }

        impl<T: Mul<Output = T> + Copy> MulAssign for $Vector<T> {
            #[inline]
            fn mul_assign(&mut self, other: Self) {
                *self = Self {
                    $( $field: self.$field * other.$field ), +
                };
            }
        }

        impl<T: Div<Output = T>> Div for $Vector<T> {
            type Output = Self;

            #[inline]
            fn div(self, other: Self) -> Self::Output {
                return Self {
                    $( $field: self.$field / other.$field ), +
                };
            }
        }

        impl<T: Div<Output = T> + Copy> DivAssign for $Vector<T> {
            #[inline]
            fn div_assign(&mut self, other: Self) {
                *self = Self {
                    $( $field: self.$field / other.$field ), +
                };
            }
        }

        impl<T: Rem<Output = T>> Rem for $Vector<T> {
            type Output = Self;

            #[inline]
            fn rem(self, other: Self) -> Self::Output {
                return Self {
                    $( $field: self.$field % other.$field ), +
                };
            }
        }

        impl<T: Rem<Output = T> + Copy> RemAssign for $Vector<T> {
            #[inline]
            fn rem_assign(&mut self, other: Self) {
                *self = Self {
                    $( $field: self.$field % other.$field ), +
                };
            }
        }

        impl<T: Neg<Output = T>> Neg for $Vector<T> {
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

        impl<T: Hash> Hash for $Vector<T> {
            #[inline]
            fn hash<H: Hasher>(&self, state: &mut H) {
                $( self.$field.hash(state); ) +
            }
        }

        impl<T: fmt::Debug> fmt::Debug for $Vector<T> {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                return f.debug_struct(stringify!($Vector))
                    $( .field(stringify!($field), &self.$field) ) +
                    .finish();
            }
        }

        impl<T: fmt::Display> fmt::Display for $Vector<T> {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                // SAFETY: Provided safety for the unwrapping in return.
                if Self::LEN == 0 { return write!(f, "()"); }
        
                let mut result = String::from("(");
                $( result.push_str(format!("{}, ", &self.$field).as_str()); ) +
                
                return write!(f, "{}", result.strip_suffix(", ").unwrap().to_string() + ")");
            }
        }

        impl<T> IntoIterator for $Vector<T> {
            type Item = T;
            type IntoIter = IntoIter<T>;
        
            fn into_iter(self) -> Self::IntoIter {
                return Self::IntoIter { vec: self.to_vec() };
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