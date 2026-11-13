//! The flare settings block of a lens.

/// Graph key of the flare settings the flare pass reads this frame: a
/// [`GpuLensFlare`] uniform buffer, published by the lens pass from the
/// view's lens (all zero, so disabled, where the view has none).
pub const FLARE_SETTINGS: &str = "lens_flare_settings";

/// `LensFlare` in WGSL (`//!use lens_flare`).
pub const FLARE_SNIPPET: helio_core::shader::ShaderSnippet =
    helio_core::wgsl_snippet!("//!use lens_flare", "../shaders/lens_flare.wgsl");

/// A lens's flare settings (`LensFlare` in [`FLARE_SNIPPET`]). Quality: 0 cheap,
/// 1 high; profile: 0 spherical, 1 anamorphic.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuLensFlare {
    pub enabled: u32,
    pub quality: u32,
    pub profile: u32,
    pub ghost_count: u32,
    pub intensity: f32,
    pub threshold: f32,
    pub soft_knee: f32,
    pub ghost_intensity: f32,
    pub halo_intensity: f32,
    pub glare_intensity: f32,
    pub streak_intensity: f32,
    pub dispersion: f32,
    /// Diffraction spikes from the iris blades: gain, and length as a
    /// fraction of the image width.
    pub starburst_intensity: f32,
    pub starburst_length: f32,
    /// 0 = neutral grey ghosts; 1 = full anti-reflection coating tints.
    pub coating_strength: f32,
    /// Brighter, chromatically fringed ghost edges (0 = flat disks).
    pub ghost_rim: f32,
    /// Dirt modulating light scattered off the front element.
    pub dirt_intensity: f32,
    /// Scene lights are lens sources: flares persist and fade physically as a
    /// light leaves the frame (nonzero = on).
    pub light_sources: u32,
    pub light_intensity: f32,
    /// How far past the frame edge (fraction of the half-frame) light still
    /// reaches the front element.
    pub field_margin: f32,
    /// Seconds for the response to settle after a change (0 = instant).
    pub response_time: f32,
    pub _pad: [f32; 3],
}

const _: () = assert!(std::mem::size_of::<GpuLensFlare>() == 96);

impl Default for GpuLensFlare {
    /// A lens's flare as authored by default: off, with the model's usual
    /// strengths.
    fn default() -> Self {
        Self {
            enabled: 0,
            quality: 0,
            profile: 0,
            ghost_count: 4,
            intensity: 0.3,
            // Full soft knee: a smooth ramp from 0 to 2x threshold, so sources
            // never switch at a cutoff, while dim extended areas (a lit floor)
            // do not form recognisable ghost images. 0/0 is fully linear.
            threshold: 2.0,
            soft_knee: 1.0,
            ghost_intensity: 1.0,
            halo_intensity: 0.5,
            glare_intensity: 0.3,
            streak_intensity: 0.5,
            dispersion: 0.01,
            starburst_intensity: 0.3,
            starburst_length: 0.25,
            coating_strength: 0.8,
            ghost_rim: 0.5,
            dirt_intensity: 0.0,
            light_sources: 1,
            light_intensity: 1.0,
            field_margin: 0.35,
            response_time: 0.06,
            _pad: [0.0; 3],
        }
    }
}
