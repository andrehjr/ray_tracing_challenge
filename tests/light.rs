use raytracer::*;

#[test]
fn test_light_has_position_and_intensity() {
    let intensity = color!(1.0, 1.0, 1.0);
    let position = point!(0.0, 0.0, 0.0);

    let light = Light {
        intensity: intensity,
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
        intensity: color!(1.0, 1.0, 1.0),
    };

    let result = lightning(material, light, position, eyev, normalv);

    assert_eq!(result, color!(1.9, 1.9, 1.9));
}

#[test]
fn test_lightning_eye_offset_45() {
    let material = Material::default();
    let position = point!(0.0, 0.0, 0.0);

    let eyev = vector!(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
    let normalv = vector!(0.0, 0.0, -1.0);

    let light = Light {
        position: point!(0.0, 0.0, -10.0),
        intensity: color!(1.0, 1.0, 1.0),
    };

    let result = lightning(material, light, position, eyev, normalv);

    assert_eq!(result, color!(1.0, 1.0, 1.0));
}
