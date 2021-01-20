use raytracer::*;

#[test]
fn test_tuple_new() {
    let tuple = Tuple::init(1.0, 2.0, 3.0, 4.0);
    assert_eq!(tuple.x, 1.0);
    assert_eq!(tuple.y, 2.0);
    assert_eq!(tuple.z, 3.0);
    assert_eq!(tuple.w, 4.0);
}

#[test]
fn test_point_new() {
    let tuple = point(1.0, 2.0, 3.0);
    assert_eq!(tuple.x, 1.0);
    assert_eq!(tuple.y, 2.0);
    assert_eq!(tuple.z, 3.0);
    assert_eq!(tuple.w, 1.0);
}

#[test]
fn test_vector_new() {
    let vector = vector(1.0, 2.0, 3.0);
    assert_eq!(vector.x, 1.0);
    assert_eq!(vector.y, 2.0);
    assert_eq!(vector.z, 3.0);
    assert_eq!(vector.w, 0.0);
}

#[test]
fn test_tuple_eq() {
    let vector_one = vector(2.0, 1.0, 3.0);
    let vector_two = vector(2.0001, 1.0001, 3.0);
    assert_eq!(vector_one, vector_two);
}

#[test]
fn test_tuple_not_eq_vector_and_points() {}

// add
#[test]
fn test_tuple_add() {
    let vector_one = vector(2.0, 1.0, 3.0);
    let vector_two = vector(1.0, 1.0, 3.0);
    let added = vector_one.add(vector_two);
    assert_eq!(added, vector(3.0, 2.0, 6.0));
}

// sub
#[test]
fn test_tuple_sub() {
    let vector_one = vector(2.0, 1.0, 3.0);
    let vector_two = vector(1.0, 1.0, 3.0);
    let added = vector_one.sub(vector_two);
    assert_eq!(added, vector(1.0, 0.0, 0.0));
}

// negate

#[test]
fn test_tuple_negate() {
    let vector_one = vector(2.0, 1.0, 3.0);
    assert_eq!(vector_one.negate(), vector(-2.0, -1.0, -3.0));
}

// multiplicação/divisão escalar

#[test]
fn test_tuple_multiply() {
    let vector_one = vector(2.0, 1.0, 3.0);
    assert_eq!(vector_one.multiply(4.0), vector(8.0, 4.0, 12.0));
}

// magnitude

#[test]
fn test_tuple_magnitude() {
    let vector_one = vector(3.0, 4.0, 12.0);
    assert_eq!(vector_one.magnitude(), 13.0);
}

#[test]
fn test_tuple_norm() {
    let vector_one = vector(3.0, 4.0, 12.0);
    assert_eq!(vector_one.norm(),
    		   vector(3.0 / 13.0, 4.0 / 13.0, 12.0 / 13.0));
}

#[test]
fn test_tuple_dot_product() {
    let vector_one = vector(2.0, 1.0, 3.0);
    let vector_two = vector(2.0, 1.0, 3.0);
    let dot_product = vector_one.dot_product(vector_two);

    assert_eq!(dot_product, vector(4.0, 1.0, 9.0));
}

#[test]
fn test_tuple_cross_product() {
    let vector_one = vector(1.0, 2.0, 3.0);
    let vector_two = vector(2.0, 3.0, 4.0);
    let dot_product = vector_one.cross_product(vector_two);

    assert_eq!(dot_product, vector(-1.0, 2.0, -1.0));
}

#[test]
fn test_tuple_cross_product_inverse() {
    let vector_one = vector(1.0, 2.0, 3.0);
    let vector_two = vector(2.0, 3.0, 4.0);
    let dot_product = vector_two.cross_product(vector_one);

    assert_eq!(dot_product, vector(1.0, -2.0, 1.0));
}

// normalização
