use crate::material::*;
use crate::matrix::*;
use crate::point;
use crate::tuple::*;

#[derive(Debug, Clone, Copy)]
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

    pub fn transform(&self, transformation: Matrix) -> Self {
        Self {
            origin: transformation.clone() * self.origin,
            direction: transformation * self.direction,
        }
    }

    pub fn intersect(&self, sphere: &Sphere) -> Vec<Intersection> {
        let sphere_to_ray = self.origin - point!(0.0, 0.0, 0.0);
        let transformed = self.transform(sphere.transform.inverse());
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
                object: sphere.clone(),
            };
            let b = Intersection {
                t: t2,
                object: sphere.clone(),
            };

            vec![a, b]
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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

