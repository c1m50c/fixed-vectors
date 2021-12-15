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
}