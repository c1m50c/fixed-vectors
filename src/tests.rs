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
    let vec2 = Vector2::new(1, 2);
    let vec3 = Vector3::new(1, 2, 3);
    let vec4 = Vector4::new(1, 2, 3, 4);

    assert_eq!(vec2.to_array(), [1, 2]);
    assert_eq!(vec3.to_array(), [1, 2, 3]);
    assert_eq!(vec4.to_array(), [1, 2, 3, 4]);
}


#[test]
fn to_vec() {
    let vec2 = Vector2::new(1, 2);
    let vec3 = Vector3::new(1, 2, 3);
    let vec4 = Vector4::new(1, 2, 3, 4);

    assert_eq!(vec2.to_vec(), vec![1, 2]);
    assert_eq!(vec3.to_vec(), vec![1, 2, 3]);
    assert_eq!(vec4.to_vec(), vec![1, 2, 3, 4]);
}


#[test]
fn iterator() {
    let vec2 = Vector2::new(1, 2);
    let vec3 = Vector3::new(1, 2, 3);
    let vec4 = Vector4::new(1, 2, 3, 4);

    let mut sum_of_vec2 = 0;
    let mut sum_of_vec3 = 0;
    let mut sum_of_vec4 = 0;
    
    for i in vec2 { sum_of_vec2 += i; }
    for i in vec3 { sum_of_vec3 += i; }
    for i in vec4 { sum_of_vec4 += i; }

    assert_eq!(sum_of_vec2, 3);
    assert_eq!(sum_of_vec3, 6);
    assert_eq!(sum_of_vec4, 10);
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