use super::*;


#[test]
fn construction() {
    let vec2 = Vector2::new(1, 2);
    assert!((vec2.x, vec2.y) == (1, 2))
}