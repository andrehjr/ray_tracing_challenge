use crate::color::*;
use crate::material::*;
use crate::tuple::*;

#[derive(Debug)]
pub struct Light {
    pub intensity: Color,
    pub position: Tuple,
}

pub fn lightning(
    material: &Material,
    light: &Light,
    position: Tuple,
    eyev: Tuple,
    normalv: Tuple,
) -> Color {
    let effective_color = &(&material.color * &light.intensity);

    let lightv = (light.position - position).norm();

    let ambient = effective_color * material.ambient;

    let light_dot_normal = lightv * normalv;

    let mut diffuse = Color::new(0.0, 0.0, 0.0);
    let mut specular = Color::new(0.0, 0.0, 0.0);

    if light_dot_normal < 0.0 {
        return ambient + diffuse + specular;
    }

    diffuse = effective_color * material.diffuse * light_dot_normal;

    let reflectv = lightv.negate().reflect(normalv);
    let reflect_dot_eye = reflectv * eyev;

    if reflect_dot_eye <= 0.0 {
        return ambient + diffuse + specular;
    }

    let factor = reflect_dot_eye.powf(material.shininess);
    specular = &light.intensity * material.specular * factor;
    return ambient + diffuse + specular;
}
