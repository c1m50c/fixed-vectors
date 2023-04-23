use super::*;


#[test]
fn to_array() {
    let vec4 = Vector4::new(0, 0, 0, 0).to_array();
    let vec3 = Vector3::new(0, 0, 0).to_array();
    let vec2 = Vector2::new(0, 0).to_array();

    assert_eq!(vec4, [0, 0, 0, 0]);
    assert_eq!(vec3, [0, 0, 0]);
    assert_eq!(vec2, [0, 0]);
}


#[test]
fn to_tuple() {
    let vec4 = Vector4::new(0, 0, 0, 0).to_tuple();
    let vec3 = Vector3::new(0, 0, 0).to_tuple();
    let vec2 = Vector2::new(0, 0).to_tuple();

    assert_eq!(vec4, (0, 0, 0, 0));
    assert_eq!(vec3, (0, 0, 0));
    assert_eq!(vec2, (0, 0));
}


#[test]
fn map() {
    let vec4 = Vector4::new(0, 0, 0, 0).map(|i| i as f32);
    let vec3 = Vector3::new(0, 0, 0).map(|i| i as f32);
    let vec2 = Vector2::new(0, 0).map(|i| i as f32);

    assert_eq!(vec4, Vector4::new(0.0, 0.0, 0.0, 0.0));
    assert_eq!(vec3, Vector3::new(0.0, 0.0, 0.0));
    assert_eq!(vec2, Vector2::new(0.0, 0.0));
}


#[test]
fn add() {
    let vec4 = Vector4::new(1, 2, 3, 4) + Vector4::new(1, 2, 3, 4);
    let vec3 = Vector3::new(1, 2, 3) + Vector3::new(1, 2, 3);
    let vec2 = Vector2::new(1, 2) + Vector2::new(1, 2);

    assert_eq!(vec4, Vector4::new(2, 4, 6, 8));
    assert_eq!(vec3, Vector3::new(2, 4, 6));
    assert_eq!(vec2, Vector2::new(2, 4));

    let vec4 = Vector4::new(1, 2, 3, 4) + 2;
    let vec3 = Vector3::new(1, 2, 3) + 2;
    let vec2 = Vector2::new(1, 2) + 2;

    assert_eq!(vec4, Vector4::new(3, 4, 5, 6));
    assert_eq!(vec3, Vector3::new(3, 4, 5));
    assert_eq!(vec2, Vector2::new(3, 4));
}


#[test]
fn add_assign() {
    let mut vec4 = Vector4::new(1, 2, 3, 4);
    vec4 += Vector4::new(1, 2, 3, 4);

    let mut vec3 = Vector3::new(1, 2, 3);
    vec3 += Vector3::new(1, 2, 3);

    let mut vec2 = Vector2::new(1, 2);
    vec2 += Vector2::new(1, 2);

    assert_eq!(vec4, Vector4::new(2, 4, 6, 8));
    assert_eq!(vec3, Vector3::new(2, 4, 6));
    assert_eq!(vec2, Vector2::new(2, 4));

    let mut vec4 = Vector4::new(1, 2, 3, 4);
    vec4 += 2;

    let mut vec3 = Vector3::new(1, 2, 3);
    vec3 += 2;

    let mut vec2 = Vector2::new(1, 2);
    vec2 += 2;

    assert_eq!(vec4, Vector4::new(3, 4, 5, 6));
    assert_eq!(vec3, Vector3::new(3, 4, 5));
    assert_eq!(vec2, Vector2::new(3, 4));
}


#[test]
fn sub() {
    let vec4 = Vector4::new(1, 2, 3, 4) - Vector4::new(1, 2, 3, 4);
    let vec3 = Vector3::new(1, 2, 3) - Vector3::new(1, 2, 3);
    let vec2 = Vector2::new(1, 2) - Vector2::new(1, 2);

    assert_eq!(vec4, Vector4::new(0, 0, 0, 0));
    assert_eq!(vec3, Vector3::new(0, 0, 0));
    assert_eq!(vec2, Vector2::new(0, 0));

    let vec4 = Vector4::new(1, 2, 3, 4) - 2;
    let vec3 = Vector3::new(1, 2, 3) - 2;
    let vec2 = Vector2::new(1, 2) - 2;

    assert_eq!(vec4, Vector4::new(-1, 0, 1, 2));
    assert_eq!(vec3, Vector3::new(-1, 0, 1));
    assert_eq!(vec2, Vector2::new(-1, 0));
}


#[test]
fn sub_assign() {
    let mut vec4 = Vector4::new(1, 2, 3, 4);
    vec4 -= Vector4::new(1, 2, 3, 4);

    let mut vec3 = Vector3::new(1, 2, 3);
    vec3 -= Vector3::new(1, 2, 3);

    let mut vec2 = Vector2::new(1, 2);
    vec2 -= Vector2::new(1, 2);

    assert_eq!(vec4, Vector4::new(0, 0, 0, 0));
    assert_eq!(vec3, Vector3::new(0, 0, 0));
    assert_eq!(vec2, Vector2::new(0, 0));

    let mut vec4 = Vector4::new(1, 2, 3, 4);
    vec4 -= 2;

    let mut vec3 = Vector3::new(1, 2, 3);
    vec3 -= 2;

    let mut vec2 = Vector2::new(1, 2);
    vec2 -= 2;

    assert_eq!(vec4, Vector4::new(-1, 0, 1, 2));
    assert_eq!(vec3, Vector3::new(-1, 0, 1));
    assert_eq!(vec2, Vector2::new(-1, 0));
}


#[test]
fn mul() {
    let vec4 = Vector4::new(1, 2, 3, 4) * Vector4::new(1, 2, 3, 4);
    let vec3 = Vector3::new(1, 2, 3) * Vector3::new(1, 2, 3);
    let vec2 = Vector2::new(1, 2) * Vector2::new(1, 2);

    assert_eq!(vec4, Vector4::new(1, 4, 9, 16));
    assert_eq!(vec3, Vector3::new(1, 4, 9));
    assert_eq!(vec2, Vector2::new(1, 4));

    let vec4 = Vector4::new(1, 2, 3, 4) * 2;
    let vec3 = Vector3::new(1, 2, 3) * 2;
    let vec2 = Vector2::new(1, 2) * 2;

    assert_eq!(vec4, Vector4::new(2, 4, 6, 8));
    assert_eq!(vec3, Vector3::new(2, 4, 6));
    assert_eq!(vec2, Vector2::new(2, 4));
}


#[test]
fn mul_assign() {
    let mut vec4 = Vector4::new(1, 2, 3, 4);
    vec4 *= Vector4::new(1, 2, 3, 4);

    let mut vec3 = Vector3::new(1, 2, 3);
    vec3 *= Vector3::new(1, 2, 3);

    let mut vec2 = Vector2::new(1, 2);
    vec2 *= Vector2::new(1, 2);

    assert_eq!(vec4, Vector4::new(1, 4, 9, 16));
    assert_eq!(vec3, Vector3::new(1, 4, 9));
    assert_eq!(vec2, Vector2::new(1, 4));

    let mut vec4 = Vector4::new(1, 2, 3, 4);
    vec4 *= 2;

    let mut vec3 = Vector3::new(1, 2, 3);
    vec3 *= 2;

    let mut vec2 = Vector2::new(1, 2);
    vec2 *= 2;

    assert_eq!(vec4, Vector4::new(2, 4, 6, 8));
    assert_eq!(vec3, Vector3::new(2, 4, 6));
    assert_eq!(vec2, Vector2::new(2, 4));
}


#[test]
fn div() {
    let vec4 = Vector4::new(1, 2, 3, 4) / Vector4::new(1, 2, 3, 4);
    let vec3 = Vector3::new(1, 2, 3) / Vector3::new(1, 2, 3);
    let vec2 = Vector2::new(1, 2) / Vector2::new(1, 2);

    assert_eq!(vec4, Vector4::new(1, 1, 1, 1));
    assert_eq!(vec3, Vector3::new(1, 1, 1));
    assert_eq!(vec2, Vector2::new(1, 1));

    let vec4 = Vector4::new(1, 2, 3, 4) / 2;
    let vec3 = Vector3::new(1, 2, 3) / 2;
    let vec2 = Vector2::new(1, 2) / 2;

    assert_eq!(vec4, Vector4::new(0, 1, 1, 2));
    assert_eq!(vec3, Vector3::new(0, 1, 1));
    assert_eq!(vec2, Vector2::new(0, 1));
}


#[test]
fn div_assign() {
    let mut vec4 = Vector4::new(1, 2, 3, 4);
    vec4 /= Vector4::new(1, 2, 3, 4);

    let mut vec3 = Vector3::new(1, 2, 3);
    vec3 /= Vector3::new(1, 2, 3);

    let mut vec2 = Vector2::new(1, 2);
    vec2 /= Vector2::new(1, 2);

    assert_eq!(vec4, Vector4::new(1, 1, 1, 1));
    assert_eq!(vec3, Vector3::new(1, 1, 1));
    assert_eq!(vec2, Vector2::new(1, 1));

    let mut vec4 = Vector4::new(1, 2, 3, 4);
    vec4 /= 2;

    let mut vec3 = Vector3::new(1, 2, 3);
    vec3 /= 2;

    let mut vec2 = Vector2::new(1, 2);
    vec2 /= 2;

    assert_eq!(vec4, Vector4::new(0, 1, 1, 2));
    assert_eq!(vec3, Vector3::new(0, 1, 1));
    assert_eq!(vec2, Vector2::new(0, 1));
}


#[test]
fn rem() {
    let vec4 = Vector4::new(1, 2, 3, 4) % Vector4::new(2, 2, 2, 2);
    let vec3 = Vector3::new(1, 2, 3) % Vector3::new(2, 2, 2);
    let vec2 = Vector2::new(1, 2) % Vector2::new(2, 2);

    assert_eq!(vec4, Vector4::new(1, 0, 1, 0));
    assert_eq!(vec3, Vector3::new(1, 0, 1));
    assert_eq!(vec2, Vector2::new(1, 0));

    let vec4 = Vector4::new(1, 2, 3, 4) % 2;
    let vec3 = Vector3::new(1, 2, 3) % 2;
    let vec2 = Vector2::new(1, 2) % 2;

    assert_eq!(vec4, Vector4::new(1, 0, 1, 0));
    assert_eq!(vec3, Vector3::new(1, 0, 1));
    assert_eq!(vec2, Vector2::new(1, 0));
}


#[test]
fn rem_assign() {
    let mut vec4 = Vector4::new(1, 2, 3, 4);
    vec4 %= Vector4::new(2, 2, 2, 2);

    let mut vec3 = Vector3::new(1, 2, 3);
    vec3 %= Vector3::new(2, 2, 2);

    let mut vec2 = Vector2::new(1, 2);
    vec2 %= Vector2::new(2, 2);

    assert_eq!(vec4, Vector4::new(1, 0, 1, 0));
    assert_eq!(vec3, Vector3::new(1, 0, 1));
    assert_eq!(vec2, Vector2::new(1, 0));

    let mut vec4 = Vector4::new(1, 2, 3, 4);
    vec4 %= 2;

    let mut vec3 = Vector3::new(1, 2, 3);
    vec3 %= 2;

    let mut vec2 = Vector2::new(1, 2);
    vec2 %= 2;

    assert_eq!(vec4, Vector4::new(1, 0, 1, 0));
    assert_eq!(vec3, Vector3::new(1, 0, 1));
    assert_eq!(vec2, Vector2::new(1, 0));
}