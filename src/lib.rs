#[cfg(test)]
mod tests;


use core::ops::{Add, AddAssign};
use core::ops::{Sub, SubAssign};
use core::ops::{Mul, MulAssign};
use core::ops::{Div, DivAssign};

use core::cmp::PartialEq;

use core::fmt;


#[macro_export]
macro_rules! impl_vector {
    ($Vector:ident { $($field:ident), + }, $size:expr) => {
        impl<T> $Vector<T> {
            #[inline]
            pub const fn new($($field: T), +) -> $Vector<T> {
                return Self {
                    $($field: $field), +
                };
            }
        }

        impl<T: Default> Default for $Vector<T> {
            #[inline]
            fn default() -> $Vector<T> {
                return Self {
                    $($field: T::default()), +
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

            fn add(self, other: Self) -> Self::Output {
                return Self {
                    $($field: self.$field + other.$field), +
                };
            }
        }

        impl<T: Add<Output = T> + Copy> AddAssign for $Vector<T> {
            fn add_assign(&mut self, other: Self) {
                *self = Self {
                    $($field: self.$field + other.$field), +
                };
            }
        }

        impl<T: Sub<Output = T>> Sub for $Vector<T> {
            type Output = Self;

            fn sub(self, other: Self) -> Self::Output {
                return Self {
                    $($field: self.$field - other.$field), +
                };
            }
        }

        impl<T: Sub<Output = T> + Copy> SubAssign for $Vector<T> {
            fn sub_assign(&mut self, other: Self) {
                *self = Self {
                    $($field: self.$field - other.$field), +
                };
            }
        }

        impl<T: Mul<Output = T>> Mul for $Vector<T> {
            type Output = Self;

            fn mul(self, other: Self) -> Self::Output {
                return Self {
                    $($field: self.$field * other.$field), +
                };
            }
        }

        impl<T: Mul<Output = T> + Copy> MulAssign for $Vector<T> {
            fn mul_assign(&mut self, other: Self) {
                *self = Self {
                    $($field: self.$field * other.$field), +
                };
            }
        }

        impl<T: Div<Output = T>> Div for $Vector<T> {
            type Output = Self;

            fn div(self, other: Self) -> Self::Output {
                return Self {
                    $($field: self.$field / other.$field), +
                };
            }
        }

        impl<T: Div<Output = T> + Copy> DivAssign for $Vector<T> {
            fn div_assign(&mut self, other: Self) {
                *self = Self {
                    $($field: self.$field / other.$field), +
                };
            }
        }

        impl<T: PartialEq> PartialEq for $Vector<T> {
            fn eq(&self, other: &Self) -> bool {
                return $(self.$field == other.$field) && +
            }
        }

        impl<T: Eq> Eq for $Vector<T> {  }


        impl<T: fmt::Debug> fmt::Debug for $Vector<T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                return f.debug_struct(stringify!($Vector))
                    $(.field(stringify!($field), &self.$field)) +
                    .finish();
            }
        }
    }
}


/// Struct for representing a two-dimensional value.
/// 
/// ## Fields
/// ```rust
/// pub x: T
/// pub y: T
/// ```
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}


/// Struct for representing a three-dimensional value.
/// 
/// ## Fields
/// ```rust
/// pub x: T
/// pub y: T
/// pub z: T
/// ```
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}


/// Struct for representing a four-dimensional value.
/// 
/// ## Fields
/// ```rust
/// pub x: T
/// pub y: T
/// pub z: T
/// pub w: T
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