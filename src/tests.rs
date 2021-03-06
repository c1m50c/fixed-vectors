use core::hash::{Hash, Hasher};
use core::convert::TryFrom;
use super::*;


#[test]
fn new() {
    let vec2 = Vector2::new(0, 1);
    let vec3 = Vector3::new(0, 1, 2);
    let vec4 = Vector4::new(0, 1, 2, 3);

    assert_eq!(vec2.x, 0);
    assert_eq!(vec2.y, 1);

    assert_eq!(vec3.x, 0);
    assert_eq!(vec3.y, 1);
    assert_eq!(vec3.z, 2);

    assert_eq!(vec4.x, 0);
    assert_eq!(vec4.y, 1);
    assert_eq!(vec4.z, 2);
    assert_eq!(vec4.w, 3);
}


#[test]
fn default() {
    let vec2 = Vector2::<i32>::default();
    let vec3 = Vector3::<i32>::default();
    let vec4 = Vector4::<i32>::default();

    assert_eq!(vec2, Vector2::new(0, 0));
    assert_eq!(vec3, Vector3::new(0, 0, 0));
    assert_eq!(vec4, Vector4::new(0, 0, 0, 0));
}


#[test]
fn add() {
    // TODO: Replace Arithmetic Tests with a macro
    let mut vec2 = Vector2::<i32>::default();
    let mut vec3 = Vector3::<i32>::default();
    let mut vec4 = Vector4::<i32>::default();

    vec2 = vec2 + Vector2::new(1, 2);
    vec3 = vec3 + Vector3::new(1, 2, 3);
    vec4 = vec4 + Vector4::new(1, 2, 3, 4);

    vec2 += Vector2::new(1, 2);
    vec3 += Vector3::new(1, 2, 3);
    vec4 += Vector4::new(1, 2, 3, 4);

    assert_eq!(vec2, Vector2::new(2, 4));
    assert_eq!(vec3, Vector3::new(2, 4, 6));
    assert_eq!(vec4, Vector4::new(2, 4, 6, 8));
}


#[test]
fn sub() {
    // TODO: Replace Arithmetic Tests with a macro
    let mut vec2 = Vector2::<i32>::default();
    let mut vec3 = Vector3::<i32>::default();
    let mut vec4 = Vector4::<i32>::default();

    vec2 = vec2 - Vector2::new(1, 2);
    vec3 = vec3 - Vector3::new(1, 2, 3);
    vec4 = vec4 - Vector4::new(1, 2, 3, 4);

    vec2 -= Vector2::new(1, 2);
    vec3 -= Vector3::new(1, 2, 3);
    vec4 -= Vector4::new(1, 2, 3, 4);

    assert_eq!(vec2, Vector2::new(-2, -4));
    assert_eq!(vec3, Vector3::new(-2, -4, -6));
    assert_eq!(vec4, Vector4::new(-2, -4, -6, -8));
}


#[test]
fn mul() {
    // TODO: Replace Arithmetic Tests with a macro
    let mut vec2 = Vector2::<i32>::default();
    let mut vec3 = Vector3::<i32>::default();
    let mut vec4 = Vector4::<i32>::default();

    vec2 = vec2 * Vector2::new(1, 2);
    vec3 = vec3 * Vector3::new(1, 2, 3);
    vec4 = vec4 * Vector4::new(1, 2, 3, 4);

    vec2 *= Vector2::new(1, 2);
    vec3 *= Vector3::new(1, 2, 3);
    vec4 *= Vector4::new(1, 2, 3, 4);

    assert_eq!(vec2, Vector2::new(0, 0));
    assert_eq!(vec3, Vector3::new(0, 0, 0));
    assert_eq!(vec4, Vector4::new(0, 0, 0, 0));
}


#[test]
fn div() {
    // TODO: Replace Arithmetic Tests with a macro
    let mut vec2 = Vector2::<i32>::default();
    let mut vec3 = Vector3::<i32>::default();
    let mut vec4 = Vector4::<i32>::default();

    vec2 = vec2 / Vector2::new(1, 2);
    vec3 = vec3 / Vector3::new(1, 2, 3);
    vec4 = vec4 / Vector4::new(1, 2, 3, 4);

    vec2 /= Vector2::new(1, 2);
    vec3 /= Vector3::new(1, 2, 3);
    vec4 /= Vector4::new(1, 2, 3, 4);

    assert_eq!(vec2, Vector2::new(0, 0));
    assert_eq!(vec3, Vector3::new(0, 0, 0));
    assert_eq!(vec4, Vector4::new(0, 0, 0, 0));
}


#[test]
fn rem() {
    // TODO: Replace Arithmetic Tests with a macro
    let mut vec2 = Vector2::<i32>::default();
    let mut vec3 = Vector3::<i32>::default();
    let mut vec4 = Vector4::<i32>::default();

    vec2 = vec2 % Vector2::new(1, 2);
    vec3 = vec3 % Vector3::new(1, 2, 3);
    vec4 = vec4 % Vector4::new(1, 2, 3, 4);

    vec2 %= Vector2::new(1, 2);
    vec3 %= Vector3::new(1, 2, 3);
    vec4 %= Vector4::new(1, 2, 3, 4);

    assert_eq!(vec2, Vector2::new(0, 0));
    assert_eq!(vec3, Vector3::new(0, 0, 0));
    assert_eq!(vec4, Vector4::new(0, 0, 0, 0));
}


#[test]
fn neg() {
    let vec2 = Vector2::<i16>::new(4, 3);
    let vec3 = Vector3::<i16>::new(4, 3, 2);
    let vec4 = Vector4::<i16>::new(4, 3, 2, 1);

    assert_eq!(-vec2, Vector2::new(-4, -3));
    assert_eq!(-vec3, Vector3::new(-4, -3, -2));
    assert_eq!(-vec4, Vector4::new(-4, -3, -2, -1));
}


#[test]
fn eq() {
    assert_eq!(Vector2::new(0, 1), Vector2::new(0, 1));
    assert_eq!(Vector3::new(0, 1, 2), Vector3::new(0, 1, 2));
    assert_eq!(Vector4::new(0, 1, 2, 3), Vector4::new(0, 1, 2, 3));
}


#[test]
fn ne() {
    assert_ne!(Vector2::new(0, 1), Vector2::new(1, 0));
    assert_ne!(Vector3::new(0, 1, 2), Vector3::new(2, 1, 0));
    assert_ne!(Vector4::new(0, 1, 2, 3), Vector4::new(3, 2, 1, 0));
}


#[test]
fn constants() {
    assert_eq!(Vector2::<()>::LEN, 2);
    assert_eq!(Vector3::<()>::LEN, 3);
    assert_eq!(Vector4::<()>::LEN, 4);
    
    assert_eq!(Vector2::<()>::NAME, "Vector2");
    assert_eq!(Vector3::<()>::NAME, "Vector3");
    assert_eq!(Vector4::<()>::NAME, "Vector4");

    assert_eq!(Vector2::<u16>::SIZE, 4);
    assert_eq!(Vector3::<u16>::SIZE, 6);
    assert_eq!(Vector4::<u16>::SIZE, 8);
}


#[test]
fn hash() {
    // NOTE: No assertions in this test, just making sure the `Hash` trait does not panic.
    let vec2 = Vector2::new(7, 2);
    let vec3 = Vector3::new(7, 2, 0);
    let vec4 = Vector4::new(7, 2, 0, 4);

    let mut hasher = std::collections::hash_map::DefaultHasher::new();

    vec2.hash(&mut hasher);
    println!("{:x}", hasher.finish());

    vec3.hash(&mut hasher);
    println!("{:x}", hasher.finish());

    vec4.hash(&mut hasher);
    println!("{:x}", hasher.finish());
}


#[test]
fn debug() {
    let vec2 = Vector2::new(0, 1);
    let vec3 = Vector3::new(0, 1, 2);
    let vec4 = Vector4::new(0, 1, 2, 3);

    assert_eq!(format!("{:?}", vec2), "Vector2 { x: 0, y: 1 }");
    assert_eq!(format!("{:?}", vec3), "Vector3 { x: 0, y: 1, z: 2 }");
    assert_eq!(format!("{:?}", vec4), "Vector4 { x: 0, y: 1, z: 2, w: 3 }");
}


#[test]
fn display() {
    let vec2 = Vector2::new(4, 2);
    let vec3 = Vector3::new(7, 7, 7);
    let vec4 = Vector4::new(1, 3, 3, 7);

    assert_eq!(format!("{}", vec2), "(4, 2)");
    assert_eq!(format!("{}", vec3), "(7, 7, 7)");
    assert_eq!(format!("{}", vec4), "(1, 3, 3, 7)");
}


#[test]
fn to_array() {
    let arr2 = Vector2::new(1, 2).to_array();
    let arr3 = Vector3::new(1, 2, 3).to_array();
    let arr4 = Vector4::new(1, 2, 3, 4).to_array();

    assert_eq!(arr2, [1, 2]);
    assert_eq!(arr3, [1, 2, 3]);
    assert_eq!(arr4, [1, 2, 3, 4]);
}


#[test]
fn to_vec() {
    let vec2 = Vector2::new(1, 2).to_vec();
    let vec3 = Vector3::new(1, 2, 3).to_vec();
    let vec4 = Vector4::new(1, 2, 3, 4).to_vec();

    assert_eq!(vec2, vec![1, 2]);
    assert_eq!(vec3, vec![1, 2, 3]);
    assert_eq!(vec4, vec![1, 2, 3, 4]);
}


#[test]
fn iterator() {
    let vec2 = Vector2::new(1, 2);
    let vec3 = Vector3::new(1, 2, 3);
    let vec4 = Vector4::new(1, 2, 3, 4);

    let mut sum_of_vec2 = 0;
    let mut sum_of_vec3 = 0;
    let mut sum_of_vec4 = 0;
    
    for i in vec2.into_iter() { sum_of_vec2 += i; }
    for i in vec3.into_iter() { sum_of_vec3 += i; }
    for i in vec4.into_iter() { sum_of_vec4 += i; }

    assert_eq!(sum_of_vec2, 3);
    assert_eq!(sum_of_vec3, 6);
    assert_eq!(sum_of_vec4, 10);

    let mut iter2 = vec2.into_iter();
    let mut iter3 = vec3.into_iter();
    let mut iter4 = vec4.into_iter();

    assert_eq!(iter2.next(), Some(1));
    assert_eq!(iter2.next(), Some(2));
    assert_eq!(iter2.next(), None);

    assert_eq!(iter3.next(), Some(1));
    assert_eq!(iter3.next(), Some(2));
    assert_eq!(iter3.next(), Some(3));
    assert_eq!(iter3.next(), None);

    assert_eq!(iter4.next(), Some(1));
    assert_eq!(iter4.next(), Some(2));
    assert_eq!(iter4.next(), Some(3));
    assert_eq!(iter4.next(), Some(4));
    assert_eq!(iter4.next(), None);

    let mut iter2 = vec2.into_iter();
    let mut iter3 = vec3.into_iter();
    let mut iter4 = vec4.into_iter();

    assert_eq!(iter2.next_back(), Some(2));
    assert_eq!(iter2.next_back(), Some(1));
    assert_eq!(iter2.next_back(), None);

    assert_eq!(iter3.next_back(), Some(3));
    assert_eq!(iter3.next_back(), Some(2));
    assert_eq!(iter3.next_back(), Some(1));
    assert_eq!(iter3.next_back(), None);

    assert_eq!(iter4.next_back(), Some(4));
    assert_eq!(iter4.next_back(), Some(3));
    assert_eq!(iter4.next_back(), Some(2));
    assert_eq!(iter4.next_back(), Some(1));
    assert_eq!(iter4.next_back(), None);
}


#[test]
fn round() {
    let vec2 = Vector2::new(5.9, 4.3).floor();
    let vec3 = Vector3::new(5.9, 4.3, 0.1).floor();
    let vec4 = Vector4::new(5.9, 4.3, 0.1, 1.2).floor();

    assert_eq!(vec2, Vector2::new(5.0, 4.0));
    assert_eq!(vec3, Vector3::new(5.0, 4.0, 0.0));
    assert_eq!(vec4, Vector4::new(5.0, 4.0, 0.0, 1.0));

    let vec2 = Vector2::new(5.9, 4.3).ceil();
    let vec3 = Vector3::new(5.9, 4.3, 0.1).ceil();
    let vec4 = Vector4::new(5.9, 4.3, 0.1, 1.2).ceil();

    assert_eq!(vec2, Vector2::new(6.0, 5.0));
    assert_eq!(vec3, Vector3::new(6.0, 5.0, 1.0));
    assert_eq!(vec4, Vector4::new(6.0, 5.0, 1.0, 2.0));

    let vec2 = Vector2::new(5.9, 4.3).round();
    let vec3 = Vector3::new(5.9, 4.3, 0.1).round();
    let vec4 = Vector4::new(5.9, 4.3, 0.1, 1.2).round();

    assert_eq!(vec2, Vector2::new(6.0, 4.0));
    assert_eq!(vec3, Vector3::new(6.0, 4.0, 0.0));
    assert_eq!(vec4, Vector4::new(6.0, 4.0, 0.0, 1.0));
}


#[test]
fn abs() {
    let vec2 = Vector2::new(-3.0, 4.0).abs();
    let vec3 = Vector3::new(-3.0, 4.0, 5.3).abs();
    let vec4 = Vector4::new(-3.0, 4.0, 5.3, -9.87).abs();

    assert_eq!(vec2, Vector2::new(3.0, 4.0));
    assert_eq!(vec3, Vector3::new(3.0, 4.0, 5.3));
    assert_eq!(vec4, Vector4::new(3.0, 4.0, 5.3, 9.87));
}


#[test]
fn min_max() {
    let vec2 = Vector2::new(1, 2);
    let vec3 = Vector3::new(1, 2, 3);
    let vec4 = Vector4::new(1, 2, 3, 4);

    assert_eq!(vec2.min(), 1);
    assert_eq!(vec2.max(), 2);

    assert_eq!(vec3.min(), 1);
    assert_eq!(vec3.max(), 3);

    assert_eq!(vec4.min(), 1);
    assert_eq!(vec4.max(), 4);
}


#[test]
fn pow() {
    let vec2 = Vector2::new(2.0, 4.0).powi(2);
    let vec3 = Vector3::new(2.0, 4.0, 6.0).powi(2);
    let vec4 = Vector4::new(2.0, 4.0, 6.0, 8.0).powi(2);

    assert_eq!(vec2, Vector2::new(4.0, 16.0));
    assert_eq!(vec3, Vector3::new(4.0, 16.0, 36.0));
    assert_eq!(vec4, Vector4::new(4.0, 16.0, 36.0, 64.0));

    let vec2 = Vector2::new(2.0, 4.0).powf(2.0);
    let vec3 = Vector3::new(2.0, 4.0, 6.0).powf(2.0);
    let vec4 = Vector4::new(2.0, 4.0, 6.0, 8.0).powf(2.0);

    assert_eq!(vec2, Vector2::new(4.0, 16.0));
    assert_eq!(vec3, Vector3::new(4.0, 16.0, 36.0));
    assert_eq!(vec4, Vector4::new(4.0, 16.0, 36.0, 64.0));

    let vec2 = Vector2::new(2, 4).pow(2);
    let vec3 = Vector3::new(2, 4, 6).pow(2);
    let vec4 = Vector4::new(2, 4, 6, 8).pow(2);

    assert_eq!(vec2, Vector2::new(4, 16));
    assert_eq!(vec3, Vector3::new(4, 16, 36));
    assert_eq!(vec4, Vector4::new(4, 16, 36, 64));
}


#[test]
fn from() {
    // TODO: There is no [`From`] Tuple trait implemented for [`Vector`], find a way to reimplement that!
    // let vec2 = Vector2::from((1, 2));
    // let vec3 = Vector3::from((1, 2, 3));
    // let vec4 = Vector4::from((1, 2, 3, 4));
    // assert_eq!(vec2, Vector2::new(1, 2));
    // assert_eq!(vec3, Vector3::new(1, 2, 3));
    // assert_eq!(vec4, Vector4::new(1, 2, 3, 4));

    let vec2 = Vector2::from([1, 2]);
    let vec3 = Vector3::from([1, 2, 3]);
    let vec4 = Vector4::from([1, 2, 3, 4]);

    assert_eq!(vec2, Vector2::new(1, 2));
    assert_eq!(vec3, Vector3::new(1, 2, 3));
    assert_eq!(vec4, Vector4::new(1, 2, 3, 4));

    let vec2 = Vector2::try_from(vec![1, 2]).unwrap();
    let vec3 = Vector3::try_from(vec![1, 2, 3]).unwrap();
    let vec4 = Vector4::try_from(vec![1, 2, 3, 4]).unwrap();

    assert_eq!(vec2, Vector2::new(1, 2));
    assert_eq!(vec3, Vector3::new(1, 2, 3));
    assert_eq!(vec4, Vector4::new(1, 2, 3, 4));
}


#[test]
fn to_tuple() {
    let tup2 = Vector2::new(1, 2).to_tuple();
    let tup3 = Vector3::new(1, 2, 3).to_tuple();
    let tup4 = Vector4::new(1, 2, 3, 4).to_tuple();

    assert_eq!(tup2, (1, 2));
    assert_eq!(tup3, (1, 2, 3));
    assert_eq!(tup4, (1, 2, 3, 4));
}


#[test]
fn trunc() {
    let vec2 = Vector2::new(1.33, 9.65).trunc();
    let vec3 = Vector3::new(1.33, 9.65, 3.33).trunc();
    let vec4 = Vector4::new(1.33, 9.65, 3.33, 7.87).trunc();

    assert_eq!(vec2, Vector2::new(1.0, 9.0));
    assert_eq!(vec3, Vector3::new(1.0, 9.0, 3.0));
    assert_eq!(vec4, Vector4::new(1.0, 9.0, 3.0, 7.0));
}


#[test]
fn fract() {
    /*
        NOTE: I hate floating-point arithmetic.
        Be careful what numbers you put in this test.
    */
    
    let vec2 = Vector2::new(1.5, 9.75).fract();
    let vec3 = Vector3::new(1.5, 9.75, 3.5).fract();
    let vec4 = Vector4::new(1.5, 9.75, 3.5, 7.25).fract();

    assert_eq!(vec2, Vector2::new(0.5, 0.75));
    assert_eq!(vec3, Vector3::new(0.5, 0.75, 0.5));
    assert_eq!(vec4, Vector4::new(0.5, 0.75, 0.5, 0.25));
}


#[test]
fn sqrt() {
    let vec2 = Vector2::new(64.0, 25.0).sqrt();
    let vec3 = Vector3::new(64.0, 25.0, 36.0).sqrt();
    let vec4 = Vector4::new(64.0, 25.0, 36.0, 49.0).sqrt();

    assert_eq!(vec2, Vector2::new(8.0, 5.0));
    assert_eq!(vec3, Vector3::new(8.0, 5.0, 6.0));
    assert_eq!(vec4, Vector4::new(8.0, 5.0, 6.0, 7.0));
}


#[test]
fn zero() {
    let vec2 = Vector2::new(1.0, 2.0).zero();
    let vec3 = Vector3::new(1.0, 2.0, 3.0).zero();
    let vec4 = Vector4::new(1.0, 2.0, 3.0, 4.0).zero();

    assert_eq!(vec2, Vector2::new(0.0, 0.0));
    assert_eq!(vec3, Vector3::new(0.0, 0.0, 0.0));
    assert_eq!(vec4, Vector4::new(0.0, 0.0, 0.0, 0.0));
}


#[test]
fn normalized() {
    let vec2 = Vector2::new(14.3, 7.9).normalized();
    let vec3 = Vector3::new(14.3, 7.9, 3.0).normalized();
    let vec4 = Vector4::new(14.3, 7.9, 3.0, 5.12).normalized();

    assert_eq!(vec2, Vector2::new(0.8753097187762677, 0.48356271177150456));
    assert_eq!(vec3, Vector3::new(0.8609148265237697, 0.4756102887788658, 0.1806115020679237));
    assert_eq!(vec4, Vector4::new(0.8227167217753062, 0.4545078393024419, 0.17259791365915514, 0.29456710597829144));
}


#[test]
fn lerp() {
    let vec2 = Vector2::new(1.0, 2.0).lerp(Vector2::new(2.0, 3.0), 1.0);
    let vec3 = Vector3::new(1.0, 2.0, 3.0).lerp(Vector3::new(2.0, 3.0, 4.0), 1.0);
    let vec4 = Vector4::new(1.0, 2.0, 3.0, 4.0).lerp(Vector4::new(2.0, 3.0, 4.0, 5.0), 1.0);

    assert_eq!(vec2, Vector2::new(2.0, 3.0));
    assert_eq!(vec3, Vector3::new(2.0, 3.0, 4.0));
    assert_eq!(vec4, Vector4::new(2.0, 3.0, 4.0, 5.0));
}


#[test]
fn dot() {
    let dot1 = Vector2::new(1.0, 2.0).dot(Vector2::new(2.0, 4.0));
    let dot2 = Vector3::new(1.0, 2.0, 3.0).dot(Vector3::new(2.0, 4.0, 6.0));
    let dot3 = Vector4::new(1.0, 2.0, 3.0, 4.0).dot(Vector4::new(2.0, 4.0, 6.0, 8.0));

    assert_eq!(dot1, 10.0);
    assert_eq!(dot2, 28.0);
    assert_eq!(dot3, 60.0);
}


#[test]
fn direction() {
    let from = Vector2::new(1.0, 2.0);
    let to = Vector2::new(5.0, 6.0);

    let direction = from.direction(to);

    assert_eq!(direction, Vector2::new(0.7071067811865475, 0.7071067811865475));
}


#[test]
fn bitwise() {
    let mut vec_and = Vector2::new(1, 2) & Vector2::new(1, 2);
    let mut vec_or = Vector2::new(1, 2) | Vector2::new(1, 2);
    let mut vec_xor = Vector2::new(1, 2) ^ Vector2::new(1, 2);

    vec_and &= Vector2::new(3, 4);
    vec_or |= Vector2::new(3, 4);
    vec_xor ^= Vector2::new(3, 4);

    assert_eq!(vec_and, Vector2::new(1, 0));
    assert_eq!(vec_or, Vector2::new(3, 6));
    assert_eq!(vec_xor, Vector2::new(3, 4));
}


#[test]
fn length() {
    // NOTE: If this method's test passes, `length_squared` will also work.
    let vec2_length = Vector2::new(3.33, 5.75).length();
    let vec3_length = Vector3::new(1.5, 2.0, 3.33).length();

    assert_eq!(vec2_length, 6.644651984867228);
    assert_eq!(vec3_length, 4.16400048030737);
}