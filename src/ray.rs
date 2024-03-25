use crate::material::*;
use crate::matrix::*;
use crate::point;
use crate::tuple::*;

#[derive(Debug)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        Self { origin, direction }
    }

    pub fn position(&self, time: f64) -> Tuple {
        self.origin + (self.direction * time)
    }

    pub fn transform(&self, transformation: &Matrix) -> Self {
        Self {
            origin: transformation * self.origin,
            direction: transformation * self.direction,
        }
    }

    pub fn intersect<'a>(&'a self, sphere: &'a Sphere) -> Vec<Intersection> {
        let transformed = self.transform(&sphere.transform.inverse());
        let sphere_to_ray = transformed.origin - point!(0.0, 0.0, 0.0);

        let a = transformed.direction * transformed.direction;
        let b = 2.0 * (transformed.direction * sphere_to_ray);

        // let a = self.direction * self.direction;
        // let b = 2.0 * (self.direction * sphere_to_ray);

        let c = (sphere_to_ray * sphere_to_ray) - 1.0;
        let discriminant = (b * b) - (4.0 * a * c);

        if discriminant < 0.0 {
            vec![]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            let a = Intersection {
                t: t1,
                object: sphere,
            };
            let b = Intersection {
                t: t2,
                object: sphere,
            };

            vec![a, b]
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub transform: Matrix,
    pub material: Material,
}

impl Sphere {
    pub fn normal_at(&self, world_point: Tuple) -> Tuple {
        //        (point - point!(0.0, 0.0, 0.0)).norm()

        let object_point = self.transform.inverse() * world_point;
        let object_normal = object_point - point!(0.0, 0.0, 0.0);
        let mut world_normal = self.transform.inverse().transpose() * object_normal;
        // hacky
        world_normal.w = 0.0;
        world_normal.norm()
    }

    pub fn init() -> Sphere {
        Sphere {
            transform: Matrix::identity(4),
            material: Material::default(),
        }
    }

    //    pub fn set_transform(&mut self, matrix: Matrix) {
    //    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Sphere,
}

impl Intersection<'_> {
    pub fn prepare_computations(&self, ray: &Ray) -> Computation {
        let point = ray.position(self.t);
        let eyev = ray.direction.negate();
        let mut normalv = self.object.normal_at(point);
        let mut inside = false;
        if normalv * eyev < 0.0 {
            inside = true;
            normalv = normalv.negate();
        }
        Computation {
            t: self.t,
            object: &self.object,
            point,
            eyev,
            normalv,
            inside,
        }
    }
}

pub fn hit<'a>(intersections: &'a Vec<Intersection>) -> Option<&'a Intersection<'a>> {
    // take the first positive intersection
    // and return the minimum of those
    intersections
        .iter()
        .filter(|x| x.t.is_sign_positive())
        .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
}

pub struct Computation<'a> {
    pub t: f64,
    pub object: &'a Sphere,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color;
    use crate::color::Color;
    use crate::light::Light;
    use crate::matrix;
    use crate::point;
    use crate::vector;
    use crate::world::World;
    use core::f64::consts::PI;

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

        let comps = intersection.prepare_computations(&ray);

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

        let comps = intersection.prepare_computations(&ray);
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

        let comps = intersection.prepare_computations(&ray);
        let color = world.shade_hit(&comps);

        assert_eq!(
            color,
            Color {
                red: 0.38066,
                green: 0.47583,
                blue: 0.2855
            }
        );
    }

    // test shading an intersection from the inside
    #[test]
    fn test_shade_intersection_inside() {
        let mut world = World::default();
        world.lights = vec![Light {
            intensity: color::WHITE,
            position: point!(0.0, 0.25, 0.0),
        }];

        let ray = Ray {
            origin: point!(0.0, 0.0, 0.0),
            direction: vector!(0.0, 0.0, 1.0),
        };

        let sphere = &world.objects[1];
        let intersection = Intersection {
            t: 0.5,
            object: sphere,
        };

        let comps = intersection.prepare_computations(&ray);
        let color = world.shade_hit(&comps);

        assert_eq!(
            color,
            Color {
                red: 0.90498,
                green: 0.90498,
                blue: 0.90498
            }
        );
    }

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
}
