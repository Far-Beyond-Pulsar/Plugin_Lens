// The lens's flare settings, one block per view: `helio_lens_api::GpuLensFlare`.
// Authored on the camera's lens, resolved by the lens pass with the lens
// itself, read by the flare pass. Never blended.
struct LensFlare {
    enabled: u32,
    quality: u32,
    profile: u32,
    ghost_count: u32,
    intensity: f32,
    threshold: f32,
    soft_knee: f32,
    ghost_intensity: f32,
    halo_intensity: f32,
    glare_intensity: f32,
    streak_intensity: f32,
    dispersion: f32,
    starburst_intensity: f32,
    starburst_length: f32,
    coating_strength: f32,
    ghost_rim: f32,
    dirt_intensity: f32,
    light_sources: u32,
    light_intensity: f32,
    field_margin: f32,
    response_time: f32,
    _pad0: f32,
    _pad1: f32,
    _pad2: f32,
}
