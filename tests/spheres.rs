use raytracer::*;
use std::f64::consts::PI;

#[test]
fn test_sphere_normal() {
    let sphere = Sphere::init();

    assert_eq!(
        sphere.normal_at(point!(1.0, 0.0, 0.0)),
        vector!(1.0, 0.0, 0.0)
    );
    assert_eq!(
        sphere.normal_at(point!(0.0, 1.0, 0.0)),
        vector!(0.0, 1.0, 0.0)
    );
    assert_eq!(
        sphere.normal_at(point!(0.0, 0.0, 1.0)),
        vector!(0.0, 0.0, 1.0)
    );

    let nonaxial = point!(
        (3.0_f64.sqrt() / 3.0),
        (3.0_f64.sqrt() / 3.0),
        (3.0_f64.sqrt() / 3.0)
    );
    let vector_result = vector!(
        (3.0_f64.sqrt() / 3.0),
        (3.0_f64.sqrt() / 3.0),
        (3.0_f64.sqrt() / 3.0)
    );

    assert_eq!(sphere.normal_at(nonaxial), vector_result);

    // assert a normalized vector is returned
    let normal = sphere.normal_at(nonaxial);
    let normalized = normal.norm();
    assert_eq!(normal, normalized);
}

#[test]
fn test_sphere_transformation() {
    let mut sphere = Sphere::init();
    sphere.transform = Matrix::identity(4).translation(0.0, 1.0, 0.0);

    let n = sphere.normal_at(point!(0.0, 1.70711, -0.70711));

    assert_eq!(n, vector!(0.0, 0.70711, -0.70711));
}

#[test]
fn test_sphere_transformation_two() {
    let mut sphere = Sphere::init();
    // order matters ?

    sphere.transform = Matrix::identity(4)
        .rotation_z(PI / 5.0)
        .scaling(1.0, 0.5, 1.0);

    let n = sphere.normal_at(point!(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0));

    assert_eq!(n, vector!(0.0, 0.97014, -0.24254));
}

#[test]
fn test_sphere_default_material() {
    let sphere = Sphere::init();
    assert_eq!(sphere.material, Material::default());
}
