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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point;
    use crate::vector;

    #[test]
    fn test_light_has_position_and_intensity() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = point!(0.0, 0.0, 0.0);

        let light = Light {
            intensity: intensity.clone(),
            position: position,
        };

        assert_eq!(light.intensity, intensity);
        assert_eq!(light.position, position);
    }

    #[test]
    fn test_lightning_eye_between_light_and_surface() {
        let material = Material::default();
        let position = point!(0.0, 0.0, 0.0);

        let eyev = vector!(0.0, 0.0, -1.0);
        let normalv = vector!(0.0, 0.0, -1.0);

        let light = Light {
            position: point!(0.0, 0.0, -10.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        };

        let result = lightning(&material, &light, position, eyev, normalv);

        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn test_lightning_eye_offset_45() {
        let material = Material::default();
        let position = point!(0.0, 0.0, 0.0);

        let eyev = vector!(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normalv = vector!(0.0, 0.0, -1.0);

        let light = Light {
            position: point!(0.0, 0.0, -10.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        };

        let result = lightning(&material, &light, position, eyev, normalv);

        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }
}
