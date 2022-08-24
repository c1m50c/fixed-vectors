use super::*;


#[test]
fn construction() {
    let vec2 = Vector2::new(1, 2);

    assert_eq!(vec2.len(), 2);
    assert_eq!((vec2.x, vec2.y), (1, 2));
}


#[test]
fn owned_transformations() {
    let arr = Vector2::new(1, 2).to_array();
    let tup = Vector2::new(1, 2).to_tuple();
    let vec = Vector2::new(1, 2).to_vec();

    assert_eq!(arr, [1, 2]);
    assert_eq!(tup, (1, 2));
    assert_eq!(vec, vec![1, 2]);
}


#[test]
fn iterators() {
    let into_iter = Vector3::new(1, 2, 3).into_iter();
    assert_eq!(into_iter.sum::<i32>(), 6);
}


#[test]
fn try_froms() {
    assert!(Vector2::try_from(vec![1]).is_err());
    assert!(Vector2::try_from(vec![1, 2]).is_ok());
    assert!(Vector2::try_from(vec![1, 2, 3]).is_ok());
}