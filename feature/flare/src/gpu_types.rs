//! Wire contract only. The flare's settings are the lens's
//! (`helio_lens_api::GpuLensFlare`, authored on the camera's lens and resolved
//! with it per view by `helio_pass_lens`); the camera's optics (f-number,
//! focal length, sensor, iris) are `helio_pass_lens::ResolvedLens`.

/// Bytes of the flare settings the pass binds (`helio_lens_api::GpuLensFlare`).
pub const FLARE_BINDING_SIZE: u64 = std::mem::size_of::<helio_lens_api::GpuLensFlare>() as u64;

/// Analytic lens sources written by `cs_sources`: a count and 32 entries.
pub const MAX_LENS_SOURCES: u64 = 32;
pub const LENS_SOURCE_SIZE: u64 = 32;
pub const LENS_SOURCES_SIZE: u64 = 16 + MAX_LENS_SOURCES * LENS_SOURCE_SIZE;
