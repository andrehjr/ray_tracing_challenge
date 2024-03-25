use crate::color::*;
use crate::light;
use crate::light::*;
use crate::material::*;
use crate::matrix::*;
use crate::point;
use crate::ray::*;
use crate::tuple::*;

// World struct contains a list of all objects in the scene and a light source
pub struct World {
    pub objects: Vec<Sphere>,
    pub lights: Vec<Light>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            lights: vec![],
        }
    }

    pub fn default() -> Self {
        let mut world = Self::new();

        let light = Light {
            position: point!(-10.0, 10.0, -10.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        };

        world.lights.push(light);

        let material = Material {
            ambient: 0.1,
            diffuse: 0.7,
            specular: 0.2,
            shininess: 200.0,
            color: Color::new(0.8, 1.0, 0.6),
        };

        let mut s1 = Sphere::init();
        s1.material = material;

        let mut s2 = Sphere::init();
        s2.transform = Matrix::identity(4).scaling(0.5, 0.5, 0.5);

        world.objects.push(s1);
        world.objects.push(s2);

        world
    }

    pub fn intersect<'a>(&'a self, ray: &'a Ray) -> Vec<Intersection> {
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

    pub fn shade_hit(&self, comps: &Computation) -> Color {
        let mut current_color = Color::new(0.0, 0.0, 0.0);
        for light in &self.lights {
            let color = light::lightning(
                &comps.object.material,
                light,
                comps.point,
                comps.eyev,
                comps.normalv,
            );
            current_color = current_color + color;
        }
        return current_color;
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let intersections = self.intersect(ray);
        if intersections.len() == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        let intersection = intersections.into_iter().find(|i| i.t >= 0.0).unwrap();
        let comps = intersection.prepare_computations(ray);
        return self.shade_hit(&comps);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray::Ray;
    use crate::vector;
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
        let outer = &mut world.objects[0];
        outer.material.ambient = 1.0;
        let inner = &mut world.objects[1];
        inner.material.ambient = 1.0;

        let ray = Ray::new(point!(0.0, 0.0, 0.75), vector!(0.0, 0.0, -1.0));

        let color = world.color_at(&ray);

        assert_eq!(color, world.objects[1].material.color);
    }
}
