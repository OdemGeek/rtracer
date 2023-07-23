use nalgebra::Vector3;

#[allow(dead_code)]
#[inline]
pub fn hash(input: u32) -> u32 {
    let state: u32 = (input.wrapping_mul(747796405u32)).wrapping_add(2891336453u32);
    let word: u32 = ((state >> ((state >> 28u32).wrapping_add(4u32))) ^ state).wrapping_mul(277803737u32);
    (word >> 22u32) ^ word
}

/// Returns random value in range 0-1
#[allow(dead_code)]
#[inline]
pub fn random_f32(seed: &mut u32) -> f32 {
    *seed = hash(*seed);
    *seed as f32 / u32::MAX as f32
}

/// Returns random vector in range 0-1
#[allow(dead_code)]
#[inline]
pub fn random_vector3(seed: &mut u32) -> Vector3<f32> {
    Vector3::new(random_f32(seed), random_f32(seed), random_f32(seed))
}

#[allow(dead_code)]
#[inline]
pub fn random_value_normal_distribution(seed: &mut u32) -> f32 {
    let theta = 2.0 * std::f32::consts::PI * random_f32(seed);
    let rho = (-2.0 * random_f32(seed).ln()).sqrt();
    rho * theta.cos()
}

#[allow(dead_code)]
#[inline]
pub fn random_direction(seed: &mut u32) -> Vector3<f32> {
    let x = random_value_normal_distribution(seed);
    let y = random_value_normal_distribution(seed);
    let z = random_value_normal_distribution(seed);
    Vector3::new(x, y, z).normalize()
}

#[allow(dead_code)]
#[inline]
pub fn random_hemisphere_direction(normal: Vector3<f32>, seed: &mut u32) -> Vector3<f32> {
    let dir = random_direction(seed);
    dir * normal.dot(&dir).is_sign_positive() as u32 as f32
}