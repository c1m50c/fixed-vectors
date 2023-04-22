#[macro_export(local_inner_macros)]
macro_rules! impl_floating_point_operations {
    ( $struct: ident { $($field: ident), + }, $size: expr, f32 ) => {
        impl $struct<f32> {
            /// Consumes the vector and returns it with all of its fields converted to their square-root as a [`f32`].
            /// 
            /// # Example
            /// 
            /// ```
            /// use fixed_vectors::Vector2;
            /// 
            /// let vec2 = Vector2::new(64.0, 25.0).sqrt_short();
            /// 
            /// assert_eq!(vec2, Vector2::new(8.0, 5.0));
            /// ```
            pub fn sqrt_short(self) -> Self {
                self.map(|f| libm::sqrtf(f))
            }

            /// Returns the magnitude of the vector as a [`f32`].
            /// 
            /// # Example
            /// 
            /// ```
            /// use fixed_vectors::Vector3;
            /// 
            /// let vec3 = Vector3::new(1.5, 2.0, 3.33);
            /// let length = vec3.length_short();
            /// 
            /// assert!(length < 4.2);
            /// ```
            pub fn length_short(&self) -> f32 {
                libm::sqrtf(self.length_squared())
            }

            /// Consumes the vector and returns it as normalized vector of [`f32s`](f32).
            /// 
            /// # Example
            /// 
            /// ```
            /// use fixed_vectors::Vector2;
            /// 
            /// let vec2 = Vector2::new(14.3, 7.9).normalized_short();
            /// 
            /// assert!(vec2.x < 1.0);
            /// assert!(vec2.y < 1.0);
            /// ```
            pub fn normalized_short(self) -> Self {
                let length_squared = self.length_squared();

                if length_squared == 0.0 {
                    return Self { $( $field: 0.0 ), + };
                }

                let length = libm::sqrtf(length_squared);

                Self {
                    $( $field: self.$field / length ), +
                }
            }
        }
    };

    ( $struct: ident { $($field: ident), + }, $size: expr, f64 ) => {
        impl $struct<f64> {
            /// Consumes the vector and returns it with all of its fields converted to their square-root as a [`f64`].
            /// 
            /// # Example
            /// 
            /// ```
            /// use fixed_vectors::Vector2;
            /// 
            /// let vec2 = Vector2::new(64.0, 25.0).sqrt();
            /// 
            /// assert_eq!(vec2, Vector2::new(8.0, 5.0));
            /// ```
            pub fn sqrt(self) -> Self {
                self.map(|f| libm::sqrt(f))
            }

            /// Returns the magnitude of the vector as a [`f64`].
            /// 
            /// # Example
            /// 
            /// ```
            /// use fixed_vectors::Vector3;
            /// 
            /// let vec3 = Vector3::new(1.5, 2.0, 3.33);
            /// let length = vec3.length();
            /// 
            /// assert!(length < 4.2);
            /// ```
            pub fn length(&self) -> f64 {
                libm::sqrt(self.length_squared())
            }

            /// Consumes the vector and returns it as normalized vector of [`f64s`](f64).
            /// 
            /// # Example
            /// 
            /// ```
            /// use fixed_vectors::Vector2;
            /// 
            /// let vec2 = Vector2::new(14.3, 7.9).normalized();
            /// 
            /// assert!(vec2.x < 1.0);
            /// assert!(vec2.y < 1.0);
            /// ```
            pub fn normalized(self) -> Self {
                let length_squared = self.length_squared();

                if length_squared == 0.0 {
                    return Self { $( $field: 0.0 ), + };
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
    ( $struct: ident { $($field: ident), + }, $size: expr ) => {
        impl<T: num_traits::float::FloatCore> $struct<T> {
            /// Returns the dot product of two vectors.
            /// 
            /// # Example
            /// 
            /// ```
            /// use fixed_vectors::Vector2;
            /// 
            /// let a = Vector2::new(1.0, 2.0);
            /// let b = Vector2::new(2.0, 4.0);
            /// let dot = a.dot(&b);
            /// 
            /// assert_eq!(dot, 10.0);
            /// ```
            #[inline]
            pub fn dot(&self, other: &Self) -> T {
                $crate::sum_repeating!(
                    $( + (self.$field * other.$field) ) +
                )
            }

            /// Returns the squared magnitude of vector.
            /// 
            /// # Example
            /// 
            /// ```
            /// use fixed_vectors::Vector3;
            /// 
            /// let vec3 = Vector3::new(3.33, 2.04, 1.337);
            /// let lsq = vec3.length_squared();
            /// 
            /// assert!(lsq >= 17.0);
            /// ```
            pub fn length_squared(&self) -> T {
                let squared = Self {
                    $( $field: self.$field * self.$field ), +
                };

                $crate::sum_repeating!(
                    $( + squared.$field ) +
                )
            }

            /// Applies [`floor`](num_traits::float::FloatCore::floor) on all fields within the vector,
            /// converting each field to the largest integer less than or equal to its value.
            /// 
            /// # Example
            /// 
            /// ```
            /// use fixed_vectors::Vector2;
            /// 
            /// let vec2 = Vector2::new(1.6, 2.3).floor();
            /// 
            /// assert_eq!(vec2, Vector2::new(1.0, 2.0));
            /// ```
            pub fn floor(self) -> Self {
                self.map(|f| f.floor())
            }

            /// Applies [`ceil`](num_traits::float::FloatCore::ceil) on all fields within the vector,
            /// converting each field to the largest integer greater than or equal to its value.
            /// 
            /// # Example
            /// 
            /// ```
            /// use fixed_vectors::Vector2;
            /// 
            /// let vec2 = Vector2::new(1.6, 2.3).ceil();
            /// 
            /// assert_eq!(vec2, Vector2::new(2.0, 3.0));
            /// ```
            pub fn ceil(self) -> Self {
                self.map(|f| f.ceil())
            }

            /// Applies [`round`](num_traits::float::FloatCore::round) on all fields within the vector,
            /// converting each field's value to its nearest integer.
            /// 
            /// # Example
            /// 
            /// ```
            /// use fixed_vectors::Vector2;
            /// 
            /// let vec2 = Vector2::new(1.6, 2.3).round();
            /// 
            /// assert_eq!(vec2, Vector2::new(2.0, 2.0));
            /// ```
            pub fn round(self) -> Self {
                self.map(|f| f.round())
            }

            /// Applies [`abs`](num_traits::float::FloatCore::abs) on all fields within the vector,
            /// converting each field to their absolute value.
            /// 
            /// # Example
            /// 
            /// ```
            /// use fixed_vectors::Vector2;
            /// 
            /// let vec2 = Vector2::new(-2.6, 2.3).abs();
            /// 
            /// assert_eq!(vec2, Vector2::new(2.6, 2.3));
            /// ```
            pub fn abs(self) -> Self {
                self.map(|f| f.abs())
            }

            /// Applies [`trunc`](num_traits::float::FloatCore::trunc) on all fields within the vector,
            /// converting each field's value to their integer parts.
            /// 
            /// # Example
            /// 
            /// ```
            /// use fixed_vectors::Vector2;
            /// 
            /// let vec2 = Vector2::new(-2.6, 2.3).trunc();
            /// 
            /// assert_eq!(vec2, Vector2::new(-2.0, 2.0));
            /// ```
            pub fn trunc(self) -> Self {
                self.map(|f| f.trunc())
            }

            /// Applies [`fract`](num_traits::float::FloatCore::fract) on all fields within the vector,
            /// converting each field's value to their fractional parts.
            /// 
            /// # Example
            /// 
            /// ```
            /// use fixed_vectors::Vector2;
            /// 
            /// let vec2 = Vector2::new(-2.5, 2.25).fract();
            /// 
            /// assert_eq!(vec2, Vector2::new(-0.5, 0.25));
            /// ```
            pub fn fract(self) -> Self {
                self.map(|f| f.fract())
            }

            /// Applies [`powi`](num_traits::float::FloatCore::powi) on all fields within the vector,
            /// raising each field's value to an integer power.
            /// 
            /// # Example
            /// 
            /// ```
            /// use fixed_vectors::Vector2;
            /// 
            /// let vec2 = Vector2::new(2.0, 4.0).powi(2);
            /// 
            /// assert_eq!(vec2, Vector2::new(4.0, 16.0));
            /// ```
            pub fn powi(self, n: i32) -> Self {
                self.map(|f| f.powi(n))
            }

            /// Linearly interpolates between two Vectors by a normalized `weight`.
            /// 
            /// # Example
            /// 
            /// ```
            /// use fixed_vectors::Vector2;
            /// 
            /// let vec2 = Vector2::new(1.0, 2.0).lerp(
            ///     Vector2::new(2.0, 3.0), 1.0
            /// );
            /// 
            /// assert_eq!(vec2, Vector2::new(2.0, 3.0));
            /// ```
            pub fn lerp(self, to: Self, weight: T) -> Self {
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