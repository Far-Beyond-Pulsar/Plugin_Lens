// The lens's blendable overrides (`helio_lens_api::overrides`).
//
// A kind that places volumes around the camera (post-process volumes)
// contributes `LensVolumeOverrides` per volume and blends them with
// `lens_overrides_blend`, in its own order, into one `LensOverrides`. The
// lens pass applies the result to the camera's own values with
// `lens_override_apply`. The result is affine in the camera's value
// (`keep * base + add`), so the blender needs no knowledge of the camera.
//
// Each `vec2` is `(keep, add)`; identity is `(1, 0)`.

/// One volume's block. `mask` bits: 1 focus distance, 2 vignette strength,
/// 4 chromatic aberration amount.
struct LensVolumeOverrides {
    mask: u32,
    focus_distance_m: f32,
    vignette_strength: f32,
    chromatic_aberration: f32,
}

/// The blended result for the view.
struct LensOverrides {
    focus_distance: vec2<f32>,
    vignette_strength: vec2<f32>,
    chromatic_aberration: vec2<f32>,
    _pad: vec2<f32>,
}

const LENS_OVERRIDE_FOCUS_DISTANCE: u32 = 1u;
const LENS_OVERRIDE_VIGNETTE_STRENGTH: u32 = 2u;
const LENS_OVERRIDE_CHROMATIC_ABERRATION: u32 = 4u;

fn lens_overrides_identity() -> LensOverrides {
    return LensOverrides(vec2(1.0, 0.0), vec2(1.0, 0.0), vec2(1.0, 0.0), vec2(0.0));
}

// Mixing a value `(keep, add)` toward `target` by `t`.
fn lens_override_mix(o: vec2<f32>, target_value: f32, t: f32) -> vec2<f32> {
    return vec2((1.0 - t) * o.x, (1.0 - t) * o.y + t * target_value);
}

fn lens_overrides_blend(acc: LensOverrides, volume: LensVolumeOverrides, t: f32) -> LensOverrides {
    var r = acc;
    if (volume.mask & LENS_OVERRIDE_FOCUS_DISTANCE) != 0u {
        r.focus_distance = lens_override_mix(acc.focus_distance, volume.focus_distance_m, t);
    }
    if (volume.mask & LENS_OVERRIDE_VIGNETTE_STRENGTH) != 0u {
        r.vignette_strength = lens_override_mix(acc.vignette_strength, volume.vignette_strength, t);
    }
    if (volume.mask & LENS_OVERRIDE_CHROMATIC_ABERRATION) != 0u {
        r.chromatic_aberration = lens_override_mix(acc.chromatic_aberration, volume.chromatic_aberration, t);
    }
    return r;
}

fn lens_override_apply(o: vec2<f32>, base: f32) -> f32 {
    return o.x * base + o.y;
}
