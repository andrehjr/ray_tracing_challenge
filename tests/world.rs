use raytracer::*;

// test default world contains default objects and light source
#[test]
fn test_default_world() {
    let world = World::default();

    assert_eq!(world.objects.len(), 2);
    let sphere = &world.objects[0];
    assert_eq!(sphere.material.color, Color::new(0.8, 1.0, 0.6));
    assert_eq!(sphere.material.ambient, 0.1);
    assert_eq!(sphere.material.diffuse, 0.7);
    assert_eq!(sphere.material.specular, 0.2);

    let sphere = &world.objects[1];
    assert_eq!(sphere.transform, Matrix::identity(4).scaling(0.5, 0.5, 0.5));

    let light = &world.lights[0];
    assert_eq!(light.position, point!(-10.0, 10.0, -10.0));
    assert_eq!(light.intensity, Color::new(1.0, 1.0, 1.0));
}

// test intersect a world with a ray
#[test]
fn test_intersect_world() {
    let world = World::default();
    let ray = Ray::new(point!(0, 0, -5), vector!(0, 0, 1));

    let intersections = world.intersect(&ray);

    assert_eq!(intersections.len(), 4);
    assert_eq!(intersections[0].t, 4.0);
    assert_eq!(intersections[1].t, 4.5);
    assert_eq!(intersections[2].t, 5.5);
    assert_eq!(intersections[3].t, 6.0);
}

// coloring tests
// test coloring when a ray misses
#[test]
fn test_color_ray_miss() {
    let world = World::default();
    let ray = Ray::new(point!(0.0, 0.0, -5.0), vector!(0.0, 1.0, 0.0));

    let color = world.color_at(&ray);

    assert_eq!(color, Color::new(0.0, 0.0, 0.0));
}

// test coloring when a ray hits
#[test]
fn test_color_ray_hit() {
    let world = World::default();
    let ray = Ray::new(point!(0.0, 0.0, -5.0), vector!(0.0, 0.0, 1.0));

    let color = world.color_at(&ray);

    assert_eq!(color, Color::new(0.38066, 0.47583, 0.2855));
}

// test coloring with an intersection behind the ray
#[test]
fn test_color_ray_behind() {
    let mut world = World::default();
    let mut outer = &mut world.objects[0];
    outer.material.ambient = 1.0;
    let mut inner = &mut world.objects[1];
    inner.material.ambient = 1.0;

    let ray = Ray::new(point!(0.0, 0.0, 0.75), vector!(0.0, 0.0, -1.0));

    let color = world.color_at(&ray);

    assert_eq!(color, world.objects[1].material.color);
}
