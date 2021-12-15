use std::ops::{Add, AddAssign};
use std::ops::{Sub, SubAssign};
use std::ops::{Mul, MulAssign};
use std::ops::{Div, DivAssign};
use std::ops::{Rem, RemAssign};

use std::ops::{Index, IndexMut};

use std::cmp::{PartialEq, Eq};

use std::marker::Copy;
use std::clone::Clone;

use std::fmt;


/// A fixed-length struct for holding a two-dimensional value.
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}


#[allow(dead_code)]
impl<T> Vector2<T> {
    #[inline(always)]
    pub const fn new(x: T, y: T) -> Self {
        return Self {
            x,
            y,
        };
    }

    #[inline(always)]
    pub fn get_x(&self) -> &T {
        return &self.x;
    }

    #[inline(always)]
    pub fn get_y(&self) -> &T {
        return &self.y;
    }

    #[inline(always)]
    pub fn get_x_mut(&mut self) -> &mut T {
        return &mut self.x;
    }

    #[inline(always)]
    pub fn get_y_mut(&mut self) -> &mut T {
        return &mut self.y;
    }

    #[inline(always)]
    pub fn set_x(&mut self, value: T) {
        self.x = value;
    }

    #[inline(always)]
    pub fn set_y(&mut self, value: T) {
        self.y = value;
    }
}


#[allow(dead_code)]
impl<T: Copy> Vector2<T> {
    #[inline(always)]
    pub fn as_tuple(&self) -> (T, T) {
        return (self.x, self.y);
    }

    #[inline(always)]
    pub fn as_array(&self) -> [T; 2] {
        return [self.x, self.y];
    }
}


impl<T: Default> Default for Vector2<T> {
    fn default() -> Self {
        return Self {
            x: T::default(),
            y: T::default(),
        };
    }
}


impl<T: fmt::Debug> fmt::Debug for Vector2<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return f.debug_struct("Vector2")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish();
    }
}


impl<T: fmt::Display> fmt::Display for Vector2<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "({}, {})", self.x, self.y);
    }
}


impl<T: PartialEq> PartialEq for Vector2<T> {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }

    fn ne(&self, other: &Self) -> bool {
        return self.x != other.x || self.y != other.y;
    }
}


impl<T: Eq> Eq for Vector2<T> {  }


impl<T: Add<Output = T>> Add for Vector2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        return Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}


impl<T: Add<Output = T> + Copy> AddAssign for Vector2<T> {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}


impl<T: Sub<Output = T>> Sub for Vector2<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        return Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}


impl<T: Sub<Output = T> + Copy> SubAssign for Vector2<T> {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}


impl<T: Mul<Output = T>> Mul for Vector2<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        return Self {
            x: self.x * other.x,
            y: self.y * other.y,
        };
    }
}


impl<T: Mul<Output = T> + Copy> MulAssign for Vector2<T> {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
        };
    }
}


impl<T: Div<Output = T>> Div for Vector2<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        return Self {
            x: self.x / other.x,
            y: self.y / other.y,
        };
    }
}


impl<T: Div<Output = T> + Copy> DivAssign for Vector2<T> {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
        };
    }
}


impl<T: Rem<Output = T>> Rem for Vector2<T> {
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        return Self {
            x: self.x % other.x,
            y: self.y % other.y,
        };
    }
}


impl<T: Rem<Output = T> + Copy> RemAssign for Vector2<T> {
    fn rem_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x % other.x,
            y: self.y % other.y,
        };
    }
}


impl<T> Index<usize> for Vector2<T> {
    type Output = T;
    
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Cannot index into Vector2, Index '{}' is out of bounds.", index),
        }
    }
}


impl<T> IndexMut<usize> for Vector2<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Cannot index into Vector2, Index '{}' is out of bounds.", index),
        }
    }
}


impl<T: Copy> Copy for Vector2<T> {  }


impl<T: Copy> Clone for Vector2<T> {
    fn clone(&self) -> Self {
        return *self;
    }
}


impl<T> From<(T, T)> for Vector2<T> {
    fn from(tup: (T, T)) -> Self {
        return Self {
            x: tup.0,
            y: tup.1,
        };
    }
}


impl<T: Copy> From<[T; 2]> for Vector2<T> {
    fn from(arr: [T; 2]) -> Self {
        return Self {
            x: arr[0],
            y: arr[1],
        };
    }
}


#[cfg(test)]
mod tests {
    use super::Vector2;

    #[test]
    fn new() {
        let vec: Vector2<f32> = Vector2::new(0.5, 0.5);
        assert!(vec.x == 0.5 && vec.y == 0.5);
    }

    #[test]
    fn default() {
        assert_eq!(Vector2::default(), Vector2::new(i32::default(), i32::default()));
    }

    #[test]
    fn eq() {
        assert_eq!(Vector2::new(0, 1), Vector2::new(0, 1));
    }

    #[test]
    fn ne() {
        assert_ne!(Vector2::new(0, 1), Vector2::new(0, 0));
    }

    #[test]
    fn add() {
        let v = Vector2::new(0, 1) + Vector2::new(1, 0);
        assert_eq!(v, Vector2::new(1, 1));

        let mut v = Vector2::new(2, 1);
        v += Vector2::new(0, 1);
        assert_eq!(v, Vector2::new(2, 2));
    }

    #[test]
    fn sub() {
        let v = Vector2::new(3, 2) - Vector2::new(1, 1);
        assert_eq!(v, Vector2::new(2, 1));

        let mut v = Vector2::new(6, 4);
        v -= Vector2::new(3, 2);
        assert_eq!(v, Vector2::new(3, 2));
    }

    #[test]
    fn mul() {
        let v = Vector2::new(6, 3) * Vector2::new(2, 2);
        assert_eq!(v, Vector2::new(12, 6));

        let mut v = Vector2::new(7, 4);
        v *= Vector2::new(3, 5);
        assert_eq!(v, Vector2::new(21, 20));
    }

    #[test]
    fn div() {
        let v = Vector2::new(12, 6) / Vector2::new(2, 2);
        assert_eq!(v, Vector2::new(6, 3));

        let mut v = Vector2::new(32, 50);
        v /= Vector2::new(4, 25);
        assert_eq!(v, Vector2::new(8, 2));
    }

    #[test]
    fn rem() {
        let v = Vector2::new(12, 6) % Vector2::new(2, 2);
        assert_eq!(v, Vector2::new(0, 0));

        let mut v = Vector2::new(32, 50);
        v %= Vector2::new(5, 9);
        assert_eq!(v, Vector2::new(2, 5));
    }

    #[test]
    fn from() {
        assert_eq!(Vector2::from((5, 3)), Vector2::new(5, 3));
        assert_eq!(Vector2::from([4, 2]), Vector2::new(4, 2));
    }

    #[test]
    fn as_tuple() {
        assert_eq!(Vector2::new(13, 37).as_tuple(), (13, 37));
    }

    #[test]
    fn as_array() {
        assert_eq!(Vector2::new(13, 37).as_array(), [13, 37]);
    }

    #[test]
    fn index() {
        let mut vec: Vector2<&str> = Vector2::new("vector", "2");
        assert_eq!(vec[0], "vector");
        assert_eq!(vec[1], "2");
        
        vec[0] = "new";
        vec[1] = "values";
        assert_eq!(vec[0], "new");
        assert_eq!(vec[1], "values");
    }
}