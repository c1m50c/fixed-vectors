macro_rules! impl_floating_vector {
    ($struct: ident { $($field: ident), + }, $len: expr) => {
        #[allow(dead_code)]
        impl<T: num_traits::Float> $struct<T> {
            fn zero(self) -> Self {
                return Self {
                    $( $field: T::zero() ), +
                };
            }
            
            fn floor(self) -> Self {
                return Self {
                    $( $field: self.$field.floor() ), +
                };
            }

            fn ceil(self) -> Self {
                return Self {
                    $( $field: self.$field.ceil() ), +
                };
            }

            fn round(self) -> Self {
                return Self {
                    $( $field: self.$field.round() ), +
                };
            }

            fn abs(self) -> Self {
                return Self {
                    $( $field: self.$field.abs() ), +
                };
            }

            fn trunc(self) -> Self {
                return Self {
                    $( $field: self.$field.trunc() ), +
                };
            }

            fn fract(self) -> Self {
                return Self {
                    $( $field: self.$field.fract() ), +
                };
            }

            fn sqrt(self) -> Self {
                return Self {
                    $( $field: self.$field.sqrt() ), +
                };
            }

            fn powi(self, n: i32) -> Self {
                return Self {
                    $( $field: self.$field.powi(n) ), +
                };
            }

            fn powf(self, n: T) -> Self {
                return Self {
                    $( $field: self.$field.powf(n) ), +
                };
            }

            fn lerp(self, to: Self, weight: T) -> Self {
                return Self {
                    $( $field: self.$field + (weight * (to.$field - self.$field)) ), +
                };
            }
        }
    }
}

pub(crate) use impl_floating_vector;