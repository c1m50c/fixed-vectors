use std::ops::{Add, AddAssign};
use std::ops::{Sub, SubAssign};
use std::ops::{Mul, MulAssign};
use std::ops::{Div, DivAssign};
use std::cmp::{PartialEq, Eq};
use std::fmt;


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
}