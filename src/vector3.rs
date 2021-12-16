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


/// A fixed-length struct for holding a three-dimensional value.
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}


#[allow(dead_code)]
impl<T> Vector3<T> {
    #[inline(always)]
    pub const fn new(x: T, y: T, z: T) -> Self {
        return Self {
            x,
            y,
            z,
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
    pub fn get_z(&self) -> &T {
        return &self.z;
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
    pub fn get_z_mut(&mut self) -> &mut T {
        return &mut self.z;
    }

    #[inline(always)]
    pub fn set_x(&mut self, value: T) {
        self.x = value;
    }

    #[inline(always)]
    pub fn set_y(&mut self, value: T) {
        self.y = value;
    }

    #[inline(always)]
    pub fn set_z(&mut self, value: T) {
        self.z = value;
    }
}


#[allow(dead_code)]
impl<T: Copy> Vector3<T> {
    #[inline(always)]
    pub fn as_tuple(&self) -> (T, T, T) {
        return (self.x, self.y, self.z);
    }

    #[inline(always)]
    pub fn as_array(&self) -> [T; 3] {
        return [self.x, self.y, self.z];
    }
}


impl<T: Default> Default for Vector3<T> {
    fn default() -> Self {
        return Self {
            x: T::default(),
            y: T::default(),
            z: T::default(),
        };
    }
}


impl<T: fmt::Debug> fmt::Debug for Vector3<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return f.debug_struct("Vector3")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .finish();
    }
}


impl<T: fmt::Display> fmt::Display for Vector3<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "({}, {}, {})", self.x, self.y, self.z);
    }
}


impl<T: PartialEq> PartialEq for Vector3<T> {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y && self.z == other.z;
    }

    fn ne(&self, other: &Self) -> bool {
        return self.x != other.x || self.y != other.y || self.z != other.z;
    }
}


impl<T: Eq> Eq for Vector3<T> {  }


impl<T: Add<Output = T>> Add for Vector3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        return Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}


impl<T: Add<Output = T> + Copy> AddAssign for Vector3<T> {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}


impl<T: Sub<Output = T>> Sub for Vector3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        return Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}


impl<T: Sub<Output = T> + Copy> SubAssign for Vector3<T> {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}


impl<T: Mul<Output = T>> Mul for Vector3<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        return Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        };
    }
}


impl<T: Mul<Output = T> + Copy> MulAssign for Vector3<T> {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        };
    }
}


impl<T: Div<Output = T>> Div for Vector3<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        return Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        };
    }
}


impl<T: Div<Output = T> + Copy> DivAssign for Vector3<T> {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        };
    }
}


impl<T: Rem<Output = T>> Rem for Vector3<T> {
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        return Self {
            x: self.x % other.x,
            y: self.y % other.y,
            z: self.z % other.z,
        };
    }
}


impl<T: Rem<Output = T> + Copy> RemAssign for Vector3<T> {
    fn rem_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x % other.x,
            y: self.y % other.y,
            z: self.z % other.z,
        };
    }
}


impl<T> Index<usize> for Vector3<T> {
    type Output = T;
    
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Cannot index into Vector3, Index '{}' is out of bounds.", index),
        }
    }
}


impl<T> IndexMut<usize> for Vector3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Cannot index into Vector3, Index '{}' is out of bounds.", index),
        }
    }
}


impl<T: Copy> Copy for Vector3<T> {  }


impl<T: Copy> Clone for Vector3<T> {
    fn clone(&self) -> Self {
        return *self;
    }
}


impl<T> From<(T, T, T)> for Vector3<T> {
    fn from(tup: (T, T, T)) -> Self {
        return Self {
            x: tup.0,
            y: tup.1,
            z: tup.2,
        };
    }
}


impl<T: Copy> From<[T; 3]> for Vector3<T> {
    fn from(arr: [T; 3]) -> Self {
        return Self {
            x: arr[0],
            y: arr[1],
            z: arr[2],
        };
    }
}


#[cfg(test)]
mod tests {
    use super::Vector3;

    #[test]
    fn new() {
        let vec: Vector3<f32> = Vector3::new(0.5, 0.5, 0.6);
        assert!(vec.x == 0.5 && vec.y == 0.5 && vec.z == 0.6);
    }

    #[test]
    fn default() {
        assert_eq!(Vector3::default(), Vector3::new(i32::default(), i32::default(), i32::default()));
    }

    #[test]
    fn eq() {
        assert_eq!(Vector3::new(0, 1, 2), Vector3::new(0, 1, 2));
    }

    #[test]
    fn ne() {
        assert_ne!(Vector3::new(0, 1, 3), Vector3::new(0, 0, 2));
    }

    #[test]
    fn add() {
        let v = Vector3::new(0, 1, 2) + Vector3::new(1, 0, 2);
        assert_eq!(v, Vector3::new(1, 1, 4));

        let mut v = Vector3::new(2, 1, 3);
        v += Vector3::new(0, 1, 3);
        assert_eq!(v, Vector3::new(2, 2, 6));
    }

    #[test]
    fn sub() {
        let v = Vector3::new(3, 2, 5) - Vector3::new(1, 1, 4);
        assert_eq!(v, Vector3::new(2, 1, 1));

        let mut v = Vector3::new(6, 4, 7);
        v -= Vector3::new(3, 2, 4);
        assert_eq!(v, Vector3::new(3, 2, 3));
    }

    #[test]
    fn mul() {
        let v = Vector3::new(6, 3, 8) * Vector3::new(2, 2, 2);
        assert_eq!(v, Vector3::new(12, 6, 16));

        let mut v = Vector3::new(7, 4, 9);
        v *= Vector3::new(3, 5, 3);
        assert_eq!(v, Vector3::new(21, 20, 27));
    }

    #[test]
    fn div() {
        let v = Vector3::new(12, 6, 18) / Vector3::new(2, 2, 2);
        assert_eq!(v, Vector3::new(6, 3, 9));

        let mut v = Vector3::new(32, 50, 96);
        v /= Vector3::new(4, 25, 8);
        assert_eq!(v, Vector3::new(8, 2, 12));
    }

    #[test]
    fn rem() {
        let v = Vector3::new(12, 6, 19) % Vector3::new(2, 2, 11);
        assert_eq!(v, Vector3::new(0, 0, 8));

        let mut v = Vector3::new(32, 50, 27);
        v %= Vector3::new(5, 9, 3);
        assert_eq!(v, Vector3::new(2, 5, 0));
    }

    #[test]
    fn from() {
        assert_eq!(Vector3::from((5, 3, 7)), Vector3::new(5, 3, 7));
        assert_eq!(Vector3::from([4, 2, 9]), Vector3::new(4, 2, 9));
    }

    #[test]
    fn as_tuple() {
        assert_eq!(Vector3::new(13, 37, 0).as_tuple(), (13, 37, 0));
    }

    #[test]
    fn as_array() {
        assert_eq!(Vector3::new(13, 37, 0).as_array(), [13, 37, 0]);
    }

    #[test]
    fn index() {
        let mut vec: Vector3<&str> = Vector3::new("we", "big", "chillin");
        assert_eq!(vec[0], "we");
        assert_eq!(vec[1], "big");
        assert_eq!(vec[2], "chillin");
        
        vec[0] = "some";
        vec[1] = "new";
        vec[2] = "values";
        assert_eq!(vec[0], "some");
        assert_eq!(vec[1], "new");
        assert_eq!(vec[2], "values");
    }
}