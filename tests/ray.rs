use raytracer::*;

#[test]
fn test_ray_new() {
    let ray = Ray {
        origin: point!(0.0, 0.0, 0.0),
        direction: vector!(0.0, 1.0, 0.0),
    };

    assert_eq!(ray.origin, point!(0.0, 0.0, 0.0));
    assert_eq!(ray.direction, vector!(0.0, 1.0, 0.0));
}

#[test]
fn test_position() {
    let ray = Ray {
        origin: point!(2.0, 3.0, 4.0),
        direction: vector!(1.0, 0.0, 0.0),
    };

    assert_eq!(ray.position(0.0), point!(2.0, 3.0, 4.0));
    assert_eq!(ray.position(1.0), point!(3.0, 3.0, 4.0));
    assert_eq!(ray.position(2.0), point!(4.0, 3.0, 4.0));

    assert_eq!(ray.position(2.5), point!(4.5, 3.0, 4.0));

    assert_eq!(ray.position(-1.0), point!(1.0, 3.0, 4.0));
}

#[test]
fn test_intersection() {
    let charles = Ray {
        origin: point!(0.0, 0.0, -5.0),
        direction: vector!(0.0, 0.0, 1.0),
    };

    let sphere = Sphere::init();
    let intersections = charles.intersect(&sphere);

    let a = Intersection {
        t: 4.0,
        object: &sphere,
    };
    let b = Intersection {
        t: 6.0,
        object: &sphere,
    };

    assert_eq!(intersections, vec![a, b]);
    assert_eq!(intersections.len(), 2);
}

#[test]
fn test_intersection_one() {
    let charles = Ray {
        origin: point!(0.0, 1.0, -5.0),
        direction: vector!(0.0, 0.0, 1.0),
    };

    let sphere = Sphere::init();
    let intersections = charles.intersect(&sphere);

    let a = Intersection {
        t: 5.0,
        object: &sphere,
    };
    let b = Intersection {
        t: 5.0,
        object: &sphere,
    };

    assert_eq!(intersections, vec![a, b]);
    assert_eq!(intersections.len(), 2);
}

#[test]
fn test_intersection_none() {
    let charles = Ray {
        origin: point!(0.0, 2.0, -5.0),
        direction: vector!(0.0, 0.0, 1.0),
    };
    let sphere = Sphere::init();
    let intersections = charles.intersect(&sphere);

    assert_eq!(intersections.len(), 0);
}

#[test]
fn test_intersection_inside_sphere() {
    let charles = Ray {
        origin: point!(0.0, 0.0, 0.0),
        direction: vector!(0.0, 0.0, 1.0),
    };

    let sphere = Sphere::init();
    let intersections = charles.intersect(&sphere);

    let a = Intersection {
        t: -1.0,
        object: &sphere,
    };
    let b = Intersection {
        t: 1.0,
        object: &sphere,
    };

    assert_eq!(intersections, vec![a, b]);
    assert_eq!(intersections.len(), 2);
}

#[test]
fn test_intersection_sphere_behind_ray() {
    let charles = Ray {
        origin: point!(0.0, 0.0, 5.0),
        direction: vector!(0.0, 0.0, 1.0),
    };

    let sphere = Sphere::init();
    let intersections = charles.intersect(&sphere);

    let a = Intersection {
        t: -6.0,
        object: &sphere,
    };
    let b = Intersection {
        t: -4.0,
        object: &sphere,
    };

    assert_eq!(intersections, vec![a, b]);
    assert_eq!(intersections.len(), 2);
}

#[test]
fn test_intersection_obj() {
    let sphere = Sphere::init();
    let intersect = Intersection {
        t: 3.5,
        object: &sphere,
    };

    assert_eq!(intersect.t, 3.5);
    assert_eq!(intersect.object, &sphere);
}

#[test]
fn test_hit() {
    let i1 = Intersection {
        t: 1.0,
        object: &Sphere::init(),
    };
    let i2 = Intersection {
        t: 2.0,
        object: &Sphere::init(),
    };

    let intersections = vec![i1.clone(), i2];

    assert_eq!(hit(&intersections), Some(&i1));
}

#[test]
fn test_hit_negative() {
    let i1 = Intersection {
        t: -1.0,
        object: &Sphere::init(),
    };
    let i2 = Intersection {
        t: 1.0,
        object: &Sphere::init(),
    };

    let intersections = vec![i1, i2.clone()];

    assert_eq!(hit(&intersections), Some(&i2));
}

#[test]
fn test_ray_transform() {
    let charles = Ray {
        origin: point!(1.0, 2.0, 3.0),
        direction: vector!(0.0, 1.0, 0.0),
    };

    let transformation = matrix![ 1.0, 0.0, 0.0, 3.0;
	                              0.0, 1.0, 0.0, 4.0;
	                              0.0, 0.0, 1.0, 5.0;
	                              0.0, 0.0, 0.0, 1.0];

    let ray = charles.transform(&transformation);

    assert_eq!(ray.origin, point!(4.0, 6.0, 8.0));
    assert_eq!(ray.direction, vector!(0.0, 1.0, 0.0));
}

#[test]
fn test_ray_transform_scaling() {
    let charles = Ray {
        origin: point!(1.0, 2.0, 3.0),
        direction: vector!(0.0, 1.0, 0.0),
    };

    let transformation = matrix![ 2.0, 0.0, 0.0, 0.0;
                                 0.0, 3.0, 0.0, 0.0;
                                 0.0, 0.0, 4.0, 0.0;
                                 0.0, 0.0, 0.0, 1.0];

    let ray = charles.transform(&transformation);

    assert_eq!(ray.origin, point!(2.0, 6.0, 12.0));
    assert_eq!(ray.direction, vector!(0.0, 3.0, 0.0));
}

#[test]
fn test_hit_all_negative() {
    let sphere = Sphere::init();
    let i1 = Intersection {
        t: -1.0,
        object: &sphere,
    };
    let i2 = Intersection {
        t: -1.0,
        object: &sphere,
    };

    let intersections = vec![i1, i2];

    assert_eq!(hit(&intersections), None);
}

// test intersection precomputation calculations
#[test]
fn test_intersection_precomputation() {
    let ray = Ray {
        origin: point!(0.0, 0.0, -5.0),
        direction: vector!(0.0, 0.0, 1.0),
    };

    let sphere = Sphere::init();
    let intersection = Intersection {
        t: 4.0,
        object: &sphere,
    };

    let comps = intersection.prepare_computations(ray);

    assert!(comps.t == intersection.t);
    assert_eq!(comps.object, intersection.object);
    assert_eq!(comps.point, point!(0.0, 0.0, -1.0));
    assert_eq!(comps.eyev, vector!(0.0, 0.0, -1.0));
    assert_eq!(comps.normalv, vector!(0.0, 0.0, -1.0));
    assert_eq!(comps.inside, false);
}


// test intersection precomputation calculations when intersection occurs on the inside of the sphere
#[test]
fn test_intersection_precomputation_inside() {
    let ray = Ray {
        origin: point!(0.0, 0.0, 0.0),
        direction: vector!(0.0, 0.0, 1.0),
    };

    let sphere = Sphere::init();
    let intersection = Intersection {
        t: 1.0,
        object: &sphere,
    };

    let comps = intersection.prepare_computations(ray);
    assert_eq!(comps.object, intersection.object);
    assert_eq!(comps.point, point!(0.0, 0.0, 1.0));
    assert_eq!(comps.eyev, vector!(0.0, 0.0, -1.0));
    assert_eq!(comps.normalv, vector!(0.0, 0.0, -1.0));
    assert_eq!(comps.inside, true);
}

// test shading an intersection
#[test]
fn test_shade_intersection() {
    let world = World::default();
    let ray = Ray {
        origin: point!(0.0, 0.0, -5.0),
        direction: vector!(0.0, 0.0, 1.0),
    };

    let sphere = &world.objects[0];
    let intersection = Intersection {
        t: 4.0,
        object: sphere,
    };

    let comps = intersection.prepare_computations(ray);
    let color = world.shade_hit(&comps);

    assert_eq!(color, Color { red: 0.38066, green: 0.47583, blue: 0.2855});
}

// test shading an intersection from the inside
#[test]
fn test_shade_intersection_inside() {
    let mut world = World::default();
    world.light = Light { intensity: color::WHITE, position: point!(0.0, 0.25, 0.0) };

    let ray = Ray {
        origin: point!(0.0, 0.0, 0.0),
        direction: vector!(0.0, 0.0, 1.0),
    };

    let sphere = &world.objects[1];
    let intersection = Intersection {
        t: 0.5,
        object: sphere,
    };

    let comps = intersection.prepare_computations(ray);
    let color = world.shade_hit(&comps);

    assert_eq!(color, Color { red: 0.90498, green: 0.90498, blue: 0.90498});
}