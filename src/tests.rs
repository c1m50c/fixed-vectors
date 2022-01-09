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

    assert_eq!(vec2.x, 0);
    assert_eq!(vec2.y, 0);

    assert_eq!(vec3.x, 0);
    assert_eq!(vec3.y, 0);
    assert_eq!(vec3.z, 0);

    assert_eq!(vec4.x, 0);
    assert_eq!(vec4.y, 0);
    assert_eq!(vec4.z, 0);
    assert_eq!(vec4.w, 0);
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