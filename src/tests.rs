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