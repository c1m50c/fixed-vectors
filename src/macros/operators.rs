/// Macro used in implementing all the operator methods for Vectors.
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
/// 
/// let vec_a = Vector2::new(2, 4);
/// let vec_b = Vector2::new(2, 4);
/// 
/// assert_eq!(vec_a * vec_b, Vector2::new(4, 16));
/// ```
// TODO: Find a way to do the `_Assign` traits without using `Copy`.
#[macro_export]
macro_rules! impl_operators {
    ($struct: ident { $($field: ident), + }, $len: expr) => {
        impl<T: core::ops::Neg<Output = T>> core::ops::Neg for $struct<T> {
            type Output = Self;

            fn neg(self) -> Self::Output {
                return Self {
                    $( $field: -self.$field ), +
                };
            }
        }

        impl<T: core::ops::Add<Output = T>> core::ops::Add for $struct<T> {
            type Output = Self;

            fn add(self, other: Self) -> Self::Output {
                return Self {
                    $( $field: self.$field + other.$field ), +
                };
            }
        }

        impl<T: core::ops::Add<Output = T> + Copy> core::ops::AddAssign for $struct<T> {
            fn add_assign(&mut self, other: Self) {
                *self = Self {
                    $( $field: self.$field + other.$field ), +
                };
            }
        }

        impl<T: core::ops::Sub<Output = T>> core::ops::Sub for $struct<T> {
            type Output = Self;

            fn sub(self, other: Self) -> Self::Output {
                return Self {
                    $( $field: self.$field - other.$field ), +
                };
            }
        }

        impl<T: core::ops::Sub<Output = T> + Copy> core::ops::SubAssign for $struct<T> {
            fn sub_assign(&mut self, other: Self) {
                *self = Self {
                    $( $field: self.$field - other.$field ), +
                };
            }
        }

        impl<T: core::ops::Mul<Output = T>> core::ops::Mul for $struct<T> {
            type Output = Self;

            fn mul(self, other: Self) -> Self::Output {
                return Self {
                    $( $field: self.$field * other.$field ), +
                };
            }
        }

        impl<T: core::ops::Mul<Output = T> + Copy> core::ops::MulAssign for $struct<T> {
            fn mul_assign(&mut self, other: Self) {
                *self = Self {
                    $( $field: self.$field * other.$field ), +
                };
            }
        }

        impl<T: core::ops::Div<Output = T>> core::ops::Div for $struct<T> {
            type Output = Self;

            fn div(self, other: Self) -> Self::Output {
                return Self {
                    $( $field: self.$field / other.$field ), +
                };
            }
        }

        impl<T: core::ops::Div<Output = T> + Copy> core::ops::DivAssign for $struct<T> {
            fn div_assign(&mut self, other: Self) {
                *self = Self {
                    $( $field: self.$field / other.$field ), +
                };
            }
        }

        impl<T: core::ops::Rem<Output = T>> core::ops::Rem for $struct<T> {
            type Output = Self;

            fn rem(self, other: Self) -> Self::Output {
                return Self {
                    $( $field: self.$field % other.$field ), +
                };
            }
        }

        impl<T: core::ops::Rem<Output = T> + Copy> core::ops::RemAssign for $struct<T> {
            fn rem_assign(&mut self, other: Self) {
                *self = Self {
                    $( $field: self.$field % other.$field ), +
                };
            }
        }

        impl<T: core::ops::BitAnd<Output = T>> core::ops::BitAnd for $struct<T> {
            type Output = Self;

            fn bitand(self, other: Self) -> Self::Output {
                return Self {
                    $( $field: self.$field & other.$field ), +
                };
            }
        }

        impl<T: core::ops::BitAnd<Output = T> + Copy> core::ops::BitAndAssign for $struct<T> {
            fn bitand_assign(&mut self, other: Self) {
                *self = Self {
                    $( $field: self.$field & other.$field ), +
                };
            }
        }

        impl<T: core::ops::BitOr<Output = T>> core::ops::BitOr for $struct<T> {
            type Output = Self;

            fn bitor(self, other: Self) -> Self::Output {
                return Self {
                    $( $field: self.$field | other.$field ), +
                };
            }
        }

        impl<T: core::ops::BitOr<Output = T> + Copy> core::ops::BitOrAssign for $struct<T> {
            fn bitor_assign(&mut self, other: Self) {
                *self = Self {
                    $( $field: self.$field | other.$field ), +
                };
            }
        }

        impl<T: core::ops::BitXor<Output = T>> core::ops::BitXor for $struct<T> {
            type Output = Self;

            fn bitxor(self, other: Self) -> Self::Output {
                return Self {
                    $( $field: self.$field ^ other.$field ), +
                };
            }
        }

        impl<T: core::ops::BitXor<Output = T> + Copy> core::ops::BitXorAssign for $struct<T> {
            fn bitxor_assign(&mut self, other: Self) {
                *self = Self {
                    $( $field: self.$field ^ other.$field ), +
                };
            }
        }

        impl<T: PartialEq> PartialEq for $struct<T> {
            fn eq(&self, other: &Self) -> bool {
                return $( self.$field == other.$field ) && +
            }
        }

        impl<T: Eq> Eq for $struct<T> {  }
    }
}