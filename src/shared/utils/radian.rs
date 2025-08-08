/// Generates N directions evenly spaced between -π and +π radians
/// For N=4: [-π/4, π/4, 3π/4, -3π/4]
/// For N=2: [0.0, π]
pub fn generate_evenly_spaced_directions(n: usize) -> Vec<f32> {
    let mut directions = Vec::with_capacity(n);
    let angle_step = 2.0 * std::f32::consts::PI / n as f32;

    for i in 0..n {
        let angle = angle_step * i as f32;
        // Shift range from [0, 2π] to [-π, π]
        let shifted_angle = if angle > std::f32::consts::PI {
            angle - 2.0 * std::f32::consts::PI
        } else {
            angle
        };
        directions.push(shifted_angle);
    }

    directions
}
