//! Camera Lens (`pulsar.lens`): the plugin's root crate, the one place the
//! plugin registers everything it contributes.
//!
//! The plugin's other crates hold the code: the feature crates
//! (`helio-pass-lens`: the camera's physically based lens and the join that
//! publishes `camera_lenses`; `helio-pass-flare`: the flare it drives),
//! and `lens_component` (the SceneDB component, its rows, presets, script
//! methods and view overscan). None of them registers a contribution; this
//! crate imports them and submits their registrations, so this file and the
//! module below are the whole list of what the plugin brings:
//!
//! - [`runtime`]: what a game links. The component class, the Helio
//!   features, the scene rows and the view overscan (how much wider than
//!   its frame a lensed view renders).
//! - The editor (with the `editor` feature) registers nothing: the inspector
//!   edits a lens through its reflected properties.
//!
//! Every registration names this crate as its origin
//! (`pulsar_plugins::origin!()`), so the plugin gate skips all of them while
//! a project disables the plugin, and the plugin catalog lists them under
//! `pulsar.lens`. Other plugins never depend on this crate; hosts reach it
//! through the installed set or a generated game, a game without the
//! `editor` feature.

// The plugin's descriptor (`pulsar-plugin.toml`): linking this crate installs
// the plugin; a project enables it or not (`pulsar_plugins`).
pulsar_plugins::installed!("pulsar-plugin.toml");

pub use lens_component;

pub mod runtime;
