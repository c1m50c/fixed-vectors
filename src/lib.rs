#[cfg(test)]
mod tests;

pub(crate) mod traits;

use core::iter::{Iterator, IntoIterator, FusedIterator, DoubleEndedIterator, ExactSizeIterator};
use core::cmp::PartialEq;
pub use traits::*;


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
/// pub const fn new( $($field: T), + ) -> Vector<T>
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
        }

        impl<T> $crate::Vector<T, $len> for $Vector<T> {
            #[inline]
            fn name(&self) -> &'static str { return Self::NAME; }

            #[inline]
            fn to_array(self) -> [T; $len] {
                return [ $(self.$field), + ];
            }

            #[inline]
            fn to_vec(self) -> std::vec::Vec<T> {
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

        impl<T: num_traits::PrimInt> $crate::IntegerVector<T, $len> for $Vector<T> {
            #[inline]
            fn pow(self, n: u32) -> Self {
                return Self { $( $field: self.$field.pow(n) ), + };
            }
        }

        impl<T: num_traits::Float> $crate::FloatingPointVector<T, $len> for $Vector<T> {
            #[inline]
            fn floor(self) -> Self {
                return Self { $( $field: self.$field.floor() ), + };
            }

            #[inline]
            fn ceil(self) -> Self {
                return Self { $( $field: self.$field.ceil() ), + }
            }

            #[inline]
            fn round(self) -> Self {
                return Self { $( $field: self.$field.round() ), + }
            }

            #[inline]
            fn abs(self) -> Self {
                return Self { $( $field: self.$field.abs() ), + };
            }

            #[inline]
            fn powi(self, n: i32) -> Self {
                return Self { $( $field: self.$field.powi(n) ), + };
            }

            #[inline]
            fn powf(self, n: T) -> Self {
                return Self { $( $field: self.$field.powf(n) ), + };
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
                let mut vec = std::vec::Vec::with_capacity(Self::LEN);
                $( vec.push(self.$field); ) +
                return Self::IntoIter { vec };
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


impl<T> TuplableVector<T, { Vector2::<()>::LEN }> for Vector2<T> {
    type Output = (T, T);

    #[inline]
    fn to_tuple(self) -> Self::Output {
        // TODO: Find a way to do this in `impl_vector` macro, this is repetitive.
        return (self.x, self.y);
    }
}


impl<T> TuplableVector<T, { Vector3::<()>::LEN }> for Vector3<T> {
    type Output = (T, T, T);

    #[inline]
    fn to_tuple(self) -> Self::Output {
        // TODO: Find a way to do this in `impl_vector` macro, this is repetitive.
        return (self.x, self.y, self.z);
    }
}


impl<T> TuplableVector<T, { Vector4::<()>::LEN }> for Vector4<T> {
    type Output = (T, T, T, T);

    #[inline]
    fn to_tuple(self) -> Self::Output {
        // TODO: Find a way to do this in `impl_vector` macro, this is repetitive.
        return (self.x, self.y, self.z, self.w);
    }
}