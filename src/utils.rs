use sfml::system::Vector2f;

/// Normalizes a vector
pub fn normalize_vector(vector: Vector2f) -> Vector2f {
    let length = (vector.x.powi(2) + vector.y.powi(2)).sqrt();
    if length != 0.0 {
        Vector2f::new(vector.x / length, vector.y / length)
    } else {
        vector
    }
}

/// Normalizes an angle to be between 0 and 360 degrees
fn normalize_angle(mut angle: f32) -> f32 {
    while angle < 0.0 {
        angle += 360.0;
    }
    while angle >= 360.0 {
        angle -= 360.0;
    }
    angle
}

// Interpolates an angle between two angles
pub fn interpolate_angle(current_angle: f32, target_angle: f32, delta: f32) -> f32 {
    let mut diff = normalize_angle(target_angle - current_angle);

    if diff > 180.0 {
        diff -= 360.0;
    }

    let new_angle = current_angle + diff * delta;
    normalize_angle(new_angle)
}
