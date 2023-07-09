#[allow(dead_code)]
pub fn hash(input: u32) -> u32 {
    let state: u32 = (input.wrapping_mul(747796405u32)).wrapping_add(2891336453u32);
    let word: u32 = ((state >> ((state >> 28u32).wrapping_add(4u32))) ^ state).wrapping_mul(277803737u32);
    (word >> 22u32) ^ word
}

/// Returns random value in range 0-1
#[allow(dead_code)]
pub fn random_f32(seed: &mut u32) -> f32 {
    *seed = hash(seed.clone());
    *seed as f32 / u32::MAX as f32
}