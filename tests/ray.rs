use raytracer::*;

#[test]
fn test_ray_new() {
    let ray = Ray {
        origin: point!(0.0, 0.0, 0.0),
        direction: vector!(0.0, 1.0, 0.0),
    };

    assert_eq!(ray.origin, point!(0.0, 0.0, 0.0));
    assert_eq!(ray.direction, point!(0.0, 1.0, 0.0));
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

    let sphere = Sphere {};
    let intersections = charles.intersect(&sphere);

    let a = Intersection {
        t: 4.0,
        object: sphere.clone(),
    };
    let b = Intersection {
        t: 6.0,
        object: sphere.clone(),
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

    let sphere = Sphere {};
    let intersections = charles.intersect(&sphere);

    let a = Intersection {
        t: 5.0,
        object: sphere.clone(),
    };
    let b = Intersection {
        t: 5.0,
        object: sphere.clone(),
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

    let intersections = charles.intersect(&Sphere {});

    assert_eq!(intersections.len(), 0);
}

#[test]
fn test_intersection_inside_sphere() {
    let charles = Ray {
        origin: point!(0.0, 0.0, 0.0),
        direction: vector!(0.0, 0.0, 1.0),
    };

    let sphere = Sphere {};
    let intersections = charles.intersect(&sphere);

    let a = Intersection {
        t: -1.0,
        object: sphere.clone(),
    };
    let b = Intersection {
        t: 1.0,
        object: sphere.clone(),
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

    let sphere = Sphere {};
    let intersections = charles.intersect(&sphere);

    let a = Intersection {
        t: -6.0,
        object: sphere.clone(),
    };
    let b = Intersection {
        t: -4.0,
        object: sphere.clone(),
    };

    assert_eq!(intersections, vec![a, b]);
    assert_eq!(intersections.len(), 2);
}

#[test]
fn test_intersection_obj() {
    let intersect = Intersection {
        t: 3.5,
        object: Sphere {},
    };

    assert_eq!(intersect.t, 3.5);
    assert_eq!(intersect.object, Sphere {});
}

#[test]
fn test_hit() {
    let i1 = Intersection {
        t: 1.0,
        object: Sphere {},
    };
    let i2 = Intersection {
        t: 2.0,
        object: Sphere {},
    };

    let intersections = IntersectList(vec![i1.clone(), i2.clone()]);

    assert_eq!(intersections.hit(), Some(i1));
}

#[test]
fn test_hit_negative() {
    let i1 = Intersection {
        t: -1.0,
        object: Sphere {},
    };
    let i2 = Intersection {
        t: 1.0,
        object: Sphere {},
    };

    let intersections = IntersectList(vec![i1.clone(), i2.clone()]);

    assert_eq!(intersections.hit(), Some(i2));
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

    let ray = charles.transform(transformation.clone());

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

    let ray = charles.transform(transformation.clone());

    assert_eq!(ray.origin, point!(2.0, 6.0, 12.0));
    assert_eq!(ray.direction, vector!(0.0, 3.0, 0.0));
}

#[test]
fn test_hit_all_negative() {
    let i1 = Intersection {
        t: -1.0,
        object: Sphere {},
    };
    let i2 = Intersection {
        t: -1.0,
        object: Sphere {},
    };

    let intersections = IntersectList(vec![i1, i2]);

    assert_eq!(intersections.hit(), None);
}
