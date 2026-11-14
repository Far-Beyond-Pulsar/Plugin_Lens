//! The lens's rows as SceneDB components, for standalone hosts (the
//! plugin's examples, other plugins' examples): each carries its contract
//! row's exact fields (`same_layout!`), so SceneDB mirrors it into the buffer
//! the lens pass reads. Engines derive the rows from the lens component
//! (`CameraLensComponent`) instead. Behind the `host-rows` feature.

use helio_components::same_layout;
use pulsar_scenedb_derive::SceneStore;

/// A camera's lens. One row per view; see the module doc.
#[derive(SceneStore, bytemuck::Pod, bytemuck::Zeroable, Clone, Copy, Debug, PartialEq)]
#[repr(C)]
#[gpu(layout = packed, buffer = "camera_lenses")]
pub struct LensComponent {
    #[gpu]
    pub view_id: u32,
    #[gpu]
    pub enabled: u32,
    /// `FOCUS_MANUAL` or `FOCUS_AUTO_POINT`.
    #[gpu]
    pub focus_mode: u32,
    /// 0 economical, 1 high (more gather taps), 2 ultra. Also the number of
    /// wavelength samples of lateral chromatic aberration: 3, 7, 16.
    #[gpu]
    pub quality: u32,
    #[gpu]
    pub focal_length_mm: f32,
    #[gpu]
    pub f_number: f32,
    /// Transmission stop; the light lost against the f-number is applied by
    /// a `element_kind::T_STOP_LOSS` element.
    #[gpu]
    pub t_stop: f32,
    #[gpu]
    pub sensor_width_mm: f32,
    /// Zero derives it from the width at 3:2.
    #[gpu]
    pub sensor_height_mm: f32,
    #[gpu]
    pub focus_distance_m: f32,
    /// Autofocus point in uv (y down).
    #[gpu]
    pub af_u: f32,
    #[gpu]
    pub af_v: f32,
    /// Autofocus ease rate in 1/s (<= 0: instant).
    #[gpu]
    pub af_speed: f32,
    /// Largest circle of confusion diameter in pixels (<= 0: 32).
    #[gpu]
    pub max_coc_px: f32,
    /// First row of this lens's elements in the `lens_elements` buffer.
    #[gpu]
    pub element_first: u32,
    #[gpu]
    pub element_count: u32,
    /// Scheimpflug: focal plane tilt in radians about an axis in the image
    /// plane at `tilt_axis` radians from +x.
    #[gpu]
    pub tilt_angle: f32,
    #[gpu]
    pub tilt_axis: f32,
    /// Petzval field curvature in dioptres at the frame corner; the focus
    /// shifts by `field_curvature * r^2`.
    #[gpu]
    pub field_curvature: f32,
    /// Iris blade count (0 = circular); shapes the flare pupil.
    #[gpu]
    pub aperture_blades: u32,
    /// Iris rotation in radians.
    #[gpu]
    pub aperture_rotation: f32,
    /// Autofocus sampling radius in uv (<= 0: 0.02).
    #[gpu]
    pub af_radius: f32,
    /// The overscan factor (>= 1; 0 reads as 1) the camera rendering this view
    /// widened its field of view by: the image the pass is given shows
    /// `overscan` times the frame's tangent extent about the frame centre, and
    /// the pass maps it back to the frame. Author it with
    /// `ray_map::applied_overscan` from the lens and elements the
    /// camera widened by; see the README.
    #[gpu]
    pub overscan: f32,
    #[gpu]
    pub _pad1: f32,
    /// The lens's flare settings, `helio_lens_api::GpuLensFlare`'s 24 words in
    /// the array shapes reflection registers.
    #[gpu]
    pub flare_0: [u32; 8],
    #[gpu]
    pub flare_1: [u32; 8],
    #[gpu]
    pub flare_2: [u32; 8],
}
same_layout!(LensComponent, crate::LensRow, crate::CAMERA_LENSES, "camera_lenses"; view_id, enabled, focus_mode, quality, focal_length_mm, f_number, t_stop, sensor_width_mm, sensor_height_mm, focus_distance_m, af_u, af_v, af_speed, max_coc_px, element_first, element_count, tilt_angle, tilt_axis, field_curvature, aperture_blades, aperture_rotation, af_radius, overscan, _pad1);

/// One optical element in front of the sensor. Rows are indexed from
/// `LensComponent::element_first`; kind `element_kind::NONE` rows are
/// skipped, so a gap in the buffer is inert.
#[derive(SceneStore, bytemuck::Pod, bytemuck::Zeroable, Clone, Copy, Debug, PartialEq)]
#[repr(C)]
#[gpu(layout = packed, buffer = "lens_elements")]
pub struct LensElementComponent {
    /// `element_kind`.
    #[gpu]
    pub kind: u32,
    /// `region_kind`.
    #[gpu]
    pub region_kind: u32,
    /// Polygon vertex count.
    #[gpu]
    pub region_count: u32,
    #[gpu]
    pub flags: u32,
    /// Width of the region's soft edge in image-height units. Negative:
    /// derived from the aperture (see `element_kind::DIOPTER`).
    #[gpu]
    pub feather: f32,
    #[gpu]
    pub _pad0: f32,
    #[gpu]
    pub _pad1: f32,
    #[gpu]
    pub _pad2: f32,
    /// Kind-specific parameters.
    #[gpu]
    pub params: [f32; 16],
    /// Region-kind-specific geometry.
    #[gpu]
    pub region: [f32; 16],
}
same_layout!(LensElementComponent, crate::LensElementRow, crate::LENS_ELEMENTS, "lens_elements"; kind, region_kind, region_count, flags, feather, _pad0, _pad1, _pad2, params, region);

/// The row's builders, returning the component.
impl LensElementComponent {
    pub fn with_anamorphic(self, squeeze: f32) -> Self {
        crate::LensElementRow::from(self)
            .with_anamorphic(squeeze)
            .into()
    }

    pub fn with_aperture_texture(self, layer: u32, amount: f32) -> Self {
        crate::LensElementRow::from(self)
            .with_aperture_texture(layer, amount)
            .into()
    }

    pub fn with_apodization(self, sigma: f32) -> Self {
        crate::LensElementRow::from(self)
            .with_apodization(sigma)
            .into()
    }

    pub fn with_cats_eye(self, strength: f32, barrel_radius: f32) -> Self {
        crate::LensElementRow::from(self)
            .with_cats_eye(strength, barrel_radius)
            .into()
    }

    pub fn with_central_obstruction(self, ratio: f32) -> Self {
        crate::LensElementRow::from(self)
            .with_central_obstruction(ratio)
            .into()
    }

    pub fn with_diffusion_tail(self, tail: f32) -> Self {
        crate::LensElementRow::from(self)
            .with_diffusion_tail(tail)
            .into()
    }

    pub fn with_f_number_rounding(self, straight_from_f_number: f32) -> Self {
        crate::LensElementRow::from(self)
            .with_f_number_rounding(straight_from_f_number)
            .into()
    }

    pub fn with_feather(self, feather: f32) -> Self {
        crate::LensElementRow::from(self)
            .with_feather(feather)
            .into()
    }

    pub fn with_glint(self, contrast: f32) -> Self {
        crate::LensElementRow::from(self)
            .with_glint(contrast)
            .into()
    }

    pub fn with_halation_tint(self, tint: [f32; 3]) -> Self {
        crate::LensElementRow::from(self)
            .with_halation_tint(tint)
            .into()
    }

    pub fn with_iris(self, blades: u32, curvature: f32, rotation: f32) -> Self {
        crate::LensElementRow::from(self)
            .with_iris(blades, curvature, rotation)
            .into()
    }

    pub fn with_kannala_brandt(self, k: [f32; 4]) -> Self {
        crate::LensElementRow::from(self)
            .with_kannala_brandt(k)
            .into()
    }

    pub fn with_mask_transform(self, scale: f32, offset: [f32; 2], tile: bool) -> Self {
        crate::LensElementRow::from(self)
            .with_mask_transform(scale, offset, tile)
            .into()
    }

    pub fn with_mount_distance(self, metres: f32) -> Self {
        crate::LensElementRow::from(self)
            .with_mount_distance(metres)
            .into()
    }

    pub fn with_onion_rings(self, amplitude: f32, rings: f32) -> Self {
        crate::LensElementRow::from(self)
            .with_onion_rings(amplitude, rings)
            .into()
    }

    pub fn with_panini(self, d: f32, s: f32) -> Self {
        crate::LensElementRow::from(self).with_panini(d, s).into()
    }

    pub fn with_prism_falloff(self, ratio: f32) -> Self {
        crate::LensElementRow::from(self)
            .with_prism_falloff(ratio)
            .into()
    }

    pub fn with_prism_ring(self) -> Self {
        crate::LensElementRow::from(self).with_prism_ring().into()
    }

    pub fn with_prism_spin(self, radians: f32) -> Self {
        crate::LensElementRow::from(self)
            .with_prism_spin(radians)
            .into()
    }

    pub fn with_psf_amount(self, amount: f32) -> Self {
        crate::LensElementRow::from(self)
            .with_psf_amount(amount)
            .into()
    }

    pub fn with_seam_darkening(self, amount: f32) -> Self {
        crate::LensElementRow::from(self)
            .with_seam_darkening(amount)
            .into()
    }

    pub fn with_spherical_aberration(self, amount: f32, asymmetry: f32) -> Self {
        crate::LensElementRow::from(self)
            .with_spherical_aberration(amount, asymmetry)
            .into()
    }

    pub fn with_swirl(self, swirl: f32) -> Self {
        crate::LensElementRow::from(self).with_swirl(swirl).into()
    }
}
