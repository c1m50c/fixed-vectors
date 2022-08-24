// TODO: Find a way to do the `_Assign` traits without using `Copy`.
macro_rules! impl_operators {
    ($struct: ident { $($field: ident), + }, $len: expr) => {
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
    }
}

pub(crate) use impl_operators;