use crate::point;
use crate::tuple::*;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection {
    pub t: f64,
    pub object: Sphere,
}

#[derive(Debug)]
pub struct IntersectList(pub Vec<Intersection>);

impl IntersectList {
    pub fn hit(&self) -> Option<Intersection> {
        // <=>
        self.0
            .iter()
            .cloned()
            .filter(|x| x.t.is_sign_positive())
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
    }
}

impl Ray {
    pub fn position(&self, time: f64) -> Tuple {
        self.origin + (self.direction * time)
    }

    pub fn intersect(&self, sphere: &Sphere) -> Vec<Intersection> {
        let sphere_to_ray = self.origin - point!(0.0, 0.0, 0.0);

        let a = self.direction * self.direction;
        let b = 2.0 * (self.direction * sphere_to_ray);
        let c = (sphere_to_ray * sphere_to_ray) - 1.0;
        let discriminant = (b * b) - (4.0 * a * c);

        if discriminant < 0.0 {
            vec![]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            let a = Intersection {
                t: t1,
                object: *sphere,
            };
            let b = Intersection {
                t: t2,
                object: *sphere,
            };

            vec![a, b]
        }
    }
}
