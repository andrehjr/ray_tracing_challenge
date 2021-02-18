use raytracer::*;
use std::f64::consts::PI;

#[test]
fn test_translation() {
    let translation = translation!(5.0, -3.0, 2.0);
    let rs = translation * point!(-3.0, 4.0, 5.0);

    assert_eq!(rs, point!(2.0, 1.0, 7.0));
}

// pending inverse matrix tests

#[test]
fn test_scaling() {
    let scaling = scaling!(2.0, 3.0, 4.0);
    let rs = scaling * point!(-4.0, 6.0, 8.0);

    assert_eq!(rs, point!(-8.0, 18.0, 32.0));
}

#[test]
fn test_scaling_reflection() {
    let scaling = scaling!(-1.0, 1.0, 1.0);
    let rs = scaling * point!(2.0, 3.0, 4.0);

    assert_eq!(rs, point!(-2.0, 3.0, 4.0));
}

// pending inverse matrix tests

// rotation x

//      y
//      |_1.0
//      |
// -------------> x
//	    |
#[test]
fn test_rotation_x() {
    let point_a = point!(0.0, 1.0, 0.0);

    let half_quarter = rotation_x!(PI / 4.0);
    let full_quarter = rotation_x!(PI / 2.0);

    assert_eq!(
        half_quarter * point_a.clone(),
        point!(0.0, 0.70710, 0.70710)
    );
    assert_eq!(full_quarter * point_a.clone(), point!(0.0, 0.0, 1.0));
}

// rotation y

#[test]
fn test_rotation_y() {
    let point_a = point!(0.0, 0.0, 1.0);

    let half_quarter = rotation_y!(PI / 4.0);
    let full_quarter = rotation_y!(PI / 2.0);

    assert_eq!(
        half_quarter * point_a.clone(),
        point!(0.70710, 0.0, 0.70710)
    );
    assert_eq!(full_quarter * point_a.clone(), point!(1.0, 0.0, 0.0));
}

// rotation z

#[test]
fn test_rotation_z() {
    let point_a = point!(0.0, 1.0, 0.0);

    let half_quarter = rotation_z!(PI / 4.0);
    let full_quarter = rotation_z!(PI / 2.0);

    assert_eq!(
        half_quarter * point_a.clone(),
        point!(-0.70710, 0.70710, 0.0)
    );
    assert_eq!(full_quarter * point_a.clone(), point!(-1.0, 0.0, 0.0));
}

// shearing / skew
#[test]
fn test_skew() {
    let skew = skew!(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let point = point!(2.0, 3.0, 4.0);

    assert_eq!(skew * point, point!(5.0, 3.0, 4.0));
}
