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

            fn normalized(self) -> Self {
                let mut iter = Self {
                    $( $field: self.$field * self.$field ), +
                }.into_iter();

                let mut magnitude = iter.next()
                    .expect("Cannot normalize a Vector that contains no fields.");
                iter.for_each(|float| magnitude = magnitude + float);

                if magnitude == T::zero() {
                    return self.zero();
                }

                let magnitude = magnitude.sqrt();
                return Self {
                    $( $field: self.$field / magnitude ), +
                };
            }

            fn dot(self, b: Self) -> T {
                let mut iter = Self {
                    $( $field: self.$field * b.$field ), +
                }.into_iter();

                let mut dot_product = iter.next()
                    .expect("Cannot normalize a Vector that contains no fields.");
                iter.for_each(|float| dot_product = dot_product * float);
                return dot_product;
            }

            fn length_squared(self) -> T {
                let mut iter = Self {
                    $( $field: self.$field * self.$field ), +
                }.into_iter();

                let mut added = iter.next()
                    .expect("Cannot normalize a Vector that contains no fields.");
                iter.for_each(|float| added = added + float);
                return added;
            }

            fn length(self) -> T {
                return self.length_squared().sqrt();
            }

            fn direction(self, to: Self) -> Self {
                todo!("Need to implement arithmetic traits first.");
                // return (to - self).normalized();
            }
        }
    }
}

pub(crate) use impl_floating_vector;