#[macro_export(local_inner_macros)]
macro_rules! impl_floating_point_operations {
    ( $struct: ident { $($field: ident), + }, $size: expr, f32 ) => {
        $crate::impl_cross_type_floating_point_operations!( $struct { $($field), + }, $size, f32 );

        impl $struct<f32> {
            pub fn sqrt(self) -> Self {
                self.map(|f| libm::sqrtf(f))
            }

            pub fn length(&self) -> f32 {
                libm::sqrtf(self.length_squared())
            }

            pub fn normalized(self) -> Self {
                let length_squared = self.length_squared();

                if length_squared == 0.0 {
                    return Self::ZERO;
                }

                let length = libm::sqrtf(length_squared);

                Self {
                    $( $field: self.$field / length ), +
                }
            }
        }
    };

    ( $struct: ident { $($field: ident), + }, $size: expr, f64 ) => {
        $crate::impl_cross_type_floating_point_operations!( $struct { $($field), + }, $size, f64 );

        impl $struct<f64> {
            pub fn sqrt(self) -> Self {
                self.map(|f| libm::sqrt(f))
            }

            pub fn length(&self) -> f64 {
                libm::sqrt(self.length_squared())
            }

            pub fn normalized(self) -> Self {
                let length_squared = self.length_squared();

                if length_squared == 0.0 {
                    return Self::ZERO;
                }

                let length = libm::sqrt(length_squared);

                Self {
                    $( $field: self.$field / length ), +
                }
            }
        }
    };
}


#[macro_export(local_inner_macros)]
macro_rules! impl_cross_type_floating_point_operations {
    ( $struct: ident { $($field: ident), + }, $size: expr, $type: ty ) => {
        impl $struct<$type> {
            pub const ZERO: Self = Self { $( $field: 0.0 ), + };

            #[inline]
            pub fn dot(self, other: Self) -> $type {
                crate::sum_repeating!(
                    $( + (self.$field * other.$field) ) +
                )
            }

            #[inline]
            pub fn squared(self) -> Self {
                Self { $( $field: self.$field * self.$field ), + }
            }

            pub fn length_squared(&self) -> $type {
                let squared = Self {
                    $( $field: self.$field * self.$field ), +
                };

                crate::sum_repeating!(
                    $( + squared.$field ) +
                )
            }

            pub fn floor(self) -> Self {
                self.map(|f| num_traits::float::FloatCore::floor(f))
            }

            pub fn ceil(self) -> Self {
                self.map(|f| num_traits::float::FloatCore::ceil(f))
            }

            pub fn round(self) -> Self {
                self.map(|f| num_traits::float::FloatCore::round(f))
            }

            pub fn abs(self) -> Self {
                self.map(|f| num_traits::float::FloatCore::abs(f))
            }

            pub fn trunc(self) -> Self {
                self.map(|f| num_traits::float::FloatCore::trunc(f))
            }

            pub fn fract(self) -> Self {
                self.map(|f| num_traits::float::FloatCore::fract(f))
            }

            pub fn powi(self, n: i32) -> Self {
                self.map(|f| num_traits::float::FloatCore::powi(f, n))
            }

            pub fn lerp(self, to: Self, weight: $type) -> Self {
                Self {
                    $( $field: self.$field + (weight * (to.$field - self.$field)) ), +
                }
            }
        }
    };
}


// HACK: Allows us to sum repeating tokens in macros.
// See: https://stackoverflow.com/a/60187870/17452730
#[macro_export(local_inner_macros)]
macro_rules! sum_repeating {
    ( + $($item: tt) * ) => {
        $($item) *
    };
}