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
fn from_array() {
    let vec2 = Vector2::from([1, 2]);
    let vec3 = Vector3::from([1, 2, 3]);
    let vec4 = Vector4::from([1, 2, 3, 4]);

    assert_eq!(vec2, Vector2::new(1, 2));
    assert_eq!(vec3, Vector3::new(1, 2, 3));
    assert_eq!(vec4, Vector4::new(1, 2, 3, 4));
}


#[test]
fn from_vec() {
    let vec2 = Vector2::from(vec![1, 2]);
    let vec3 = Vector3::from(vec![1, 2, 3]);
    let vec4 = Vector4::from(vec![1, 2, 3, 4]);

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