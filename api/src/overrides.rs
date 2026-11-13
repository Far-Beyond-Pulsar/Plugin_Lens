//! The lens's blendable overrides.
//!
//! Three artistic parameters of a lens may be overridden by volumes around
//! the camera: the manual focus distance, the vignette strength (the amount
//! of the lens's natural vignetting elements) and the chromatic aberration
//! amount (the first-order coefficient of the lens's lateral chromatic
//! aberration elements). They apply to what the camera's lens already has: a
//! lens without a vignetting or lateral aberration element, or in autofocus,
//! has nothing for that override to change.
//!
//! A blended result is affine in the camera's value, `keep * base + add`
//! ([`GpuLensOverrides`]), so the blender (the post-process volumes) needs
//! no access to the camera's lens: blending a volume's value `v` in by `t` is
//! `(keep, add) -> ((1 - t) keep, (1 - t) add + t v)`, exactly the sequential
//! `lerp(previous, v, t)` an override of a plain setting would be.

/// Graph key of the blended overrides for the view ([`GpuLensOverrides`], a
/// storage buffer). Whoever blends volumes publishes it; the lens pass reads
/// it. Without one, the lens uses its own values.
pub const LENS_OVERRIDES: &str = "lens_overrides";

/// `LensVolumeOverrides`, `LensOverrides` and their functions in WGSL
/// (`//!use lens_overrides`).
pub const OVERRIDES_SNIPPET: helio_core::shader::ShaderSnippet =
    helio_core::wgsl_snippet!("//!use lens_overrides", "../shaders/lens_overrides.wgsl");

/// [`GpuLensVolumeOverrides::mask`] bit: the focus distance.
pub const OVERRIDE_FOCUS_DISTANCE: u32 = 1;
/// [`GpuLensVolumeOverrides::mask`] bit: the vignette strength.
pub const OVERRIDE_VIGNETTE_STRENGTH: u32 = 2;
/// [`GpuLensVolumeOverrides::mask`] bit: the chromatic aberration amount.
pub const OVERRIDE_CHROMATIC_ABERRATION: u32 = 4;

/// One volume's overrides (`LensVolumeOverrides` in [`OVERRIDES_SNIPPET`]); the
/// block a volume's row embeds.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuLensVolumeOverrides {
    /// `OVERRIDE_*` bits: which of the values below override.
    pub mask: u32,
    /// Metres.
    pub focus_distance_m: f32,
    /// 0..1, the amount of natural vignetting.
    pub vignette_strength: f32,
    /// The first-order lateral aberration coefficient.
    pub chromatic_aberration: f32,
}

/// The blended overrides for a view (`LensOverrides` in [`OVERRIDES_SNIPPET`]):
/// per value `(keep, add)`, the lens's value becoming `keep * value + add`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuLensOverrides {
    pub focus_distance: [f32; 2],
    pub vignette_strength: [f32; 2],
    pub chromatic_aberration: [f32; 2],
    pub _pad: [f32; 2],
}

const _: () = assert!(std::mem::size_of::<GpuLensVolumeOverrides>() == 16);
const _: () = assert!(std::mem::size_of::<GpuLensOverrides>() == 32);

impl Default for GpuLensOverrides {
    /// Nothing overridden: every value stays the lens's.
    fn default() -> Self {
        Self::IDENTITY
    }
}

fn mix(o: [f32; 2], target: f32, t: f32) -> [f32; 2] {
    [(1.0 - t) * o[0], (1.0 - t) * o[1] + t * target]
}

impl GpuLensOverrides {
    pub const IDENTITY: Self = Self {
        focus_distance: [1.0, 0.0],
        vignette_strength: [1.0, 0.0],
        chromatic_aberration: [1.0, 0.0],
        _pad: [0.0; 2],
    };

    /// `self` with `volume` blended in by weight `t`; `lens_overrides_blend`
    /// in WGSL does the same.
    pub fn blend(self, volume: &GpuLensVolumeOverrides, t: f32) -> Self {
        let mut r = self;
        if volume.mask & OVERRIDE_FOCUS_DISTANCE != 0 {
            r.focus_distance = mix(self.focus_distance, volume.focus_distance_m, t);
        }
        if volume.mask & OVERRIDE_VIGNETTE_STRENGTH != 0 {
            r.vignette_strength = mix(self.vignette_strength, volume.vignette_strength, t);
        }
        if volume.mask & OVERRIDE_CHROMATIC_ABERRATION != 0 {
            r.chromatic_aberration = mix(self.chromatic_aberration, volume.chromatic_aberration, t);
        }
        r
    }
}

/// `o` applied to the lens's `base` value (`lens_override_apply` in WGSL).
pub fn apply(o: [f32; 2], base: f32) -> f32 {
    o[0] * base + o[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_changes_nothing() {
        assert_eq!(apply(GpuLensOverrides::IDENTITY.focus_distance, 3.5), 3.5);
    }

    #[test]
    fn blending_is_the_sequential_lerp_of_the_value() {
        let a = GpuLensVolumeOverrides {
            mask: OVERRIDE_FOCUS_DISTANCE,
            focus_distance_m: 10.0,
            ..Default::default()
        };
        let b = GpuLensVolumeOverrides {
            mask: OVERRIDE_FOCUS_DISTANCE | OVERRIDE_VIGNETTE_STRENGTH,
            focus_distance_m: 2.0,
            vignette_strength: 0.8,
            ..Default::default()
        };
        let blended = GpuLensOverrides::IDENTITY.blend(&a, 0.5).blend(&b, 0.25);
        let base = 4.0_f32;
        let direct = {
            let first = base + (10.0 - base) * 0.5;
            first + (2.0 - first) * 0.25
        };
        assert!((apply(blended.focus_distance, base) - direct).abs() < 1e-5);
        // Only `b` overrides the vignette, with weight 0.25 over the lens's 0.2.
        assert!((apply(blended.vignette_strength, 0.2) - (0.2 + (0.8 - 0.2) * 0.25)).abs() < 1e-6);
        // Nothing overrides the aberration.
        assert_eq!(apply(blended.chromatic_aberration, 0.007), 0.007);
    }

    #[test]
    fn a_masked_out_value_is_left_alone() {
        let volume = GpuLensVolumeOverrides {
            mask: 0,
            focus_distance_m: 99.0,
            ..Default::default()
        };
        assert_eq!(
            GpuLensOverrides::IDENTITY.blend(&volume, 1.0),
            GpuLensOverrides::IDENTITY
        );
    }

    /// The snippets are valid WGSL together, and their structs have the
    /// Rust rows' sizes.
    #[test]
    fn the_snippets_validate() {
        let source = format!(
            "{}\n{}\n",
            crate::FLARE_SNIPPET.source,
            OVERRIDES_SNIPPET.source
        );
        let module = naga::front::wgsl::parse_str(&source)
            .unwrap_or_else(|e| panic!("{}", e.emit_to_string(&source)));
        let mut layouter = naga::proc::Layouter::default();
        layouter.update(module.to_ctx()).unwrap();
        let mut sizes = std::collections::HashMap::new();
        for (handle, ty) in module.types.iter() {
            if let Some(name) = &ty.name {
                sizes.insert(name.clone(), layouter[handle].size);
            }
        }
        assert_eq!(sizes["LensFlare"], 96);
        assert_eq!(sizes["LensVolumeOverrides"], 16);
        assert_eq!(sizes["LensOverrides"], 32);
    }
}
