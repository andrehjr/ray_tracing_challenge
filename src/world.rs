use crate::color;
use crate::light::*;
use crate::material::*;
use crate::matrix::*;
use crate::ray::*;
use crate::point;
use crate::tuple::*;

// World struct contains a list of all objects in the scene and a light source
pub struct World {
    pub objects: Vec<Sphere>,
    pub light: Light,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            light: Light {
                intensity: color!(1.0, 1.0, 1.0),
                position: point!(-10.0, 10.0, -10.0),
            },
        }
    }

    pub fn default() -> Self {
        let mut world = Self::new();

        let material = Material {
            ambient: 0.1,
            diffuse: 0.7,
            specular: 0.2,
            shininess: 200.0,
            color: color!(0.8, 1.0, 0.6),
        };

        let mut s1 = Sphere::init();
        s1.material = material;

        let mut s2 = Sphere::init();
        s2.transform = Matrix::identity(4).scaling(0.5, 0.5, 0.5);

        world.objects.push(s1);
        world.objects.push(s2);

        world
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        // iterate over all objects in the world and collect all intersections
        let mut intersections = vec![];
        for object in &self.objects {
            let mut isects = ray.intersect(object);
            intersections.append(&mut isects);
        }
        // sort intersections by t value
        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        
        intersections
    }

}