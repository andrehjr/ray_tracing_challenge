use raytracer::*;

#[test]
fn test_tuple_new() {
    let tuple = Tuple {
        x: 1.0,
        y: 2.0,
        z: 3.0,
        w: 4.0,
    };
    assert_eq!(tuple.x, 1.0);
    assert_eq!(tuple.y, 2.0);
    assert_eq!(tuple.z, 3.0);
    assert_eq!(tuple.w, 4.0);
}

#[test]
fn test_point_new() {
    let tuple = point!(1.0, 2.0, 3.0);
    assert_eq!(tuple.x, 1.0);
    assert_eq!(tuple.y, 2.0);
    assert_eq!(tuple.z, 3.0);
    assert_eq!(tuple.w, 1.0);
}

#[test]
fn test_vector_new() {
    let vector = vector!(1.0, 2.0, 3.0);
    assert_eq!(vector.x, 1.0);
    assert_eq!(vector.y, 2.0);
    assert_eq!(vector.z, 3.0);
    assert_eq!(vector.w, 0.0);
}

#[test]
fn test_tuple_eq() {
    let vector_one = vector!(2.0, 1.0, 3.0);
    let vector_two = vector!(2.0001, 1.0001, 3.0);
    assert_eq!(vector_one, vector_two);
}

#[test]
fn test_tuple_not_eq_vector_and_points() {}

// add
#[test]
fn test_tuple_add() {
    let vector_one = vector!(2.0, 1.0, 3.0);
    let vector_two = vector!(1.0, 1.0, 3.0);
    let added = vector_one + vector_two;
    assert_eq!(added, vector!(3.0, 2.0, 6.0));
}

// sub
#[test]
fn test_tuple_sub() {
    let vector_one = vector!(2.0, 1.0, 3.0);
    let vector_two = vector!(1.0, 1.0, 3.0);
    let added = vector_one - vector_two;
    assert_eq!(added, vector!(1.0, 0.0, 0.0));
}

// negate

#[test]
fn test_tuple_negate() {
    let vector_one = vector!(2.0, 1.0, 3.0);
    assert_eq!(vector_one.negate(), vector!(-2.0, -1.0, -3.0));
}

// multiplicação/divisão escalar

#[test]
fn test_tuple_multiply() {
    let vector_one = vector!(2.0, 1.0, 3.0);
    assert_eq!(vector_one * 4.0, vector!(8.0, 4.0, 12.0));
}

// magnitude

#[test]
fn test_tuple_magnitude() {
    let vector_one = vector!(3.0, 4.0, 12.0);
    assert_eq!(vector_one.magnitude(), 13.0);
}

#[test]
fn test_tuple_norm() {
    let vector_one = vector!(3.0, 4.0, 12.0);
    assert_eq!(
        vector_one.norm(),
        vector!(3.0 / 13.0, 4.0 / 13.0, 12.0 / 13.0)
    );
}

#[test]
fn test_tuple_dot_product() {
    let vector_one = vector!(2.0, 1.0, 3.0);
    let vector_two = vector!(2.0, 1.0, 3.0);
    let dot_product = vector_one * vector_two;

    assert_eq!(dot_product, vector!(4.0, 1.0, 9.0));
}

#[test]
fn test_tuple_cross_product() {
    let vector_one = vector!(1.0, 2.0, 3.0);
    let vector_two = vector!(2.0, 3.0, 4.0);
    let dot_product = vector_one.cross_product(vector_two);

    assert_eq!(dot_product, vector!(-1.0, 2.0, -1.0));
}

#[test]
fn test_tuple_cross_product_inverse() {
    let vector_one = vector!(1.0, 2.0, 3.0);
    let vector_two = vector!(2.0, 3.0, 4.0);
    let dot_product = vector_two.cross_product(vector_one);

    assert_eq!(dot_product, vector!(1.0, -2.0, 1.0));
}

#[test]
fn test_color_create() {
    let color = Color {
        red: -0.5,
        green: 0.4,
        blue: 1.7,
    };

    assert_eq!(color.red, -0.5);
    assert_eq!(color.green, 0.4);
    assert_eq!(color.blue, 1.7);
}

#[test]
fn test_color_add() {
    let color_one = color!(0.9, 0.6, 0.75);
    let color_two = color!(0.7, 0.1, 0.25);
    let color_three = color_one + color_two;

    assert_eq!(color_three, color!(1.6, 0.7, 1.0));
}

#[test]
fn test_color_sub() {
    let color_one = color!(0.9, 0.6, 0.75);
    let color_two = color!(0.7, 0.1, 0.25);
    let color_three = color_one - color_two;

    assert_eq!(color_three, color!(0.2, 0.5, 0.50));
}

#[test]
fn test_color_multiply() {
    let color_one = color!(0.2, 0.3, 0.4);
    let color_two = color_one * 2.0;

    assert_eq!(color_two, color!(0.4, 0.6, 0.8));
}

#[test]
fn test_color_hadamard_product() {
    let color_one = color!(1.0, 0.2, 0.4);
    let color_two = color!(0.9, 1.0, 0.1);
    let color_three = color_one * color_two;

    assert_eq!(color_three, color!(0.9, 0.2, 0.04));
}
