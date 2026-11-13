//! Camera lenses (`pulsar.lens`): the component crate of the plugin that
//! owns a camera's optics.
//!
//! - The component ([`component`]): `CameraLensComponent`, Helio's
//!   physically based lens as authored (optics, focus, tilt, an ordered
//!   stack of optical elements, presets), its rows and its script methods.
//! - What the engine runs for them: the plugin's Helio features
//!   ([`render_features`]: the lens, `helio_pass_lens::LensFeature`, and the
//!   flare it drives, `helio_pass_flare::LensFlareFeature`), its scene rows
//!   ([`register_columns`], [`derivation`]: the lens source rows the
//!   plugin's join publishes as `camera_lenses`, and the element rows) and
//!   how much wider than its frame a lensed view renders
//!   ([`camera_lens_view_overscan`]).
//!
//! This crate registers nothing itself but its type's SceneDB attachments
//! (GPU mirror and clear, script methods, inspector hints): the plugin's
//! root crate (`lens_plugin`) submits the component class and every
//! registration above, in one place.

pub mod component;
pub mod flare;

pub use component::*;
pub use flare::*;

/// Registers the lenses' source-row and element-row columns.
pub fn register_columns(
    store: &mut pulsar_scenedb::gpu::SceneGpuStore,
    device: &std::sync::Arc<wgpu::Device>,
) {
    register_lens_gpu_columns(store, device);
}

/// The lenses' scene rows: the plugin's join from source rows to the lens
/// pass's `camera_lenses` rows.
pub fn derivation(
    device: &wgpu::Device,
    keys: helio_scene_join::RowJoinKeys,
) -> Box<dyn helio_scene_join::SceneDerivation> {
    Box::new(helio_pass_lens::scene_rows::scene_rows(
        device,
        keys,
        helio_scene_join::BufferKey::of(CAMERA_LENS_SOURCES_BUFFER),
    ))
}

/// The lens and its flare: the lens resolves the scene colour through the
/// camera's optics after anti-aliasing, the flare adds the lights' ghosts.
pub fn render_features() -> helio_feature_api::FeatureSet {
    helio_feature_api::FeatureSet::new()
        .with(helio_pass_lens::LensFeature)
        .with(helio_pass_flare::LensFlareFeature)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_source_row_is_the_pass_row() {
        assert_eq!(
            std::mem::size_of::<CameraLensSourceRow>() as u64,
            helio_pass_lens::scene_rows::CAMERA_LENS_SOURCE_ROW_BYTES
        );
    }
}
