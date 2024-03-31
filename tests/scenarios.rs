use raytracer::{
    color::Color,
    light::Light,
    matrix::Matrix,
    point,
    ray::{Intersection, Ray, Sphere},
    tuple::Tuple,
    vector,
    world::World,
};

#[test]
// test shadowed scene
fn shade_git_given_an_intersection_in_shadow() {
    let mut w = World::new();
    w.lights.push(Light {
        position: point!(0, 0, -10),
        intensity: Color::new(1.0, 1.0, 1.0),
    });
    w.objects.push(Sphere::init());
    let mut s2 = Sphere::init();
    s2.transform = Matrix::identity(4).translation(0.0, 0.0, 10.0);
    w.objects.push(s2);
    let r = Ray::new(point!(0, 0, 5), vector!(0, 0, 1));
    let i = Intersection {
        t: 4.0,
        object: &w.objects[1],
    };
    let comps = i.prepare_computations(&r);
    let c = w.shade_hit(&comps);
    assert_eq!(c, Color::new(0.1, 0.1, 0.1));
}
