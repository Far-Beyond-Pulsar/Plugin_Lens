//! What the lens plugin registers for a game (and the editor): every
//! runtime contribution, grouped by the engine seam it plugs into.

use engine_backend::scene::{ClassSpec, ComponentsBuilder};
use engine_backend::scene::helio_bridge::SceneRowsRegistration;
use engine_backend::subsystems::render::{RenderFeatureRegistration, ViewOverscanRegistration};
use lens_component::CameraLensComponent;

/// Everything the plugin registers for a game (and the editor).
pub(crate) fn build(p: &mut pulsar_plugins::PluginBuilder) {
    // ── Component classes ───────────────────────────────────────────────────

    p.components().class::<CameraLensComponent>(ClassSpec::default());

    // ── Rendering ───────────────────────────────────────────────────────────

    // The lens resolves the scene colour through the camera's optics after
    // anti-aliasing; the flare adds the lights' ghosts.
    p.add(RenderFeatureRegistration {
        origin: p.origin(),
        features: lens_component::render_features,
        configure: |config| config,
    });

    // The lenses' source and element rows and the join that publishes them as
    // the lens pass's `camera_lenses`.
    p.add(SceneRowsRegistration {
        origin: p.origin(),
        name: "lens.rows",
        register_columns: lens_component::register_columns,
        derivation: Some(lens_component::derivation),
    });

    // A lensed view renders wider than its frame; the lens pass crops it back.
    p.add(ViewOverscanRegistration {
        origin: p.origin(),
        name: "lens.overscan",
        overscan: lens_component::camera_lens_view_overscan,
    });
}

#[cfg(test)]
mod tests {
    use engine_backend::subsystems::render::helio_renderer::render_features;

    #[test]
    fn registers_its_features() {
        let features = render_features::plugin_features();
        for id in ["helio.lens", "helio.lens_flare"] {
            assert!(
                features.ids().contains(&id),
                "{id}: features {:?}",
                features.ids()
            );
        }
    }

    /// Every registration of the plugin names this root, so it resolves to
    /// the plugin's id.
    #[test]
    fn every_registration_resolves_to_the_plugin() {
        let ours: Vec<_> = pulsar_plugins::contributions()
            .into_iter()
            .filter(|summary| summary.origin == pulsar_plugins::origin!())
            .collect();
        assert!(
            ours.iter()
                .any(|summary| summary.category == "component.class")
        );
        for summary in &ours {
            assert_eq!(
                pulsar_plugins::plugin_of(summary.origin).map(|plugin| plugin.id.as_str()),
                Some("pulsar.lens"),
                "{summary:?}"
            );
        }
    }
}
