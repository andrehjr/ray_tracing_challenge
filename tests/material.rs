use raytracer::*;

#[test]
fn test_default_material() {
    let m = Material {
        color: Color::new(1.0, 1.0, 1.0),
        ambient: 0.1,
        diffuse: 0.9,
        specular: 0.9,
        shininess: 200.0,
    };

    assert_eq!(m.ambient, 0.1);
}
