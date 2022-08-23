use super::*;


#[test]
fn construction() {
    let vec2 = Vector2::new(1, 2);

    assert!(vec2.len() == 2);
    assert!((vec2.x, vec2.y) == (1, 2));
}


#[test]
fn owned_transformation() {
    let arr = Vector2::new(1, 2).to_array();
    let vec = Vector2::new(1, 2).to_vec();

    assert_eq!(arr, [1, 2]);
    assert_eq!(vec, vec![1, 2]);
}