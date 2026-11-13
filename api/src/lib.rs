//! The lens's public contract (`pulsar.lens`).
//!
//! - [`flare`]: the lens's flare settings. Authored on the camera's lens,
//!   resolved with it per view by the lens pass, read by the flare pass. The
//!   physical prescription (optics, elements, flare model) belongs to the
//!   camera and never blends.
//! - [`overrides`]: the lens's blendable artistic parameters (focus
//!   distance, vignette strength, chromatic aberration amount). A kind that
//!   places volumes around the camera blends its volumes' blocks into one
//!   [`GpuLensOverrides`] under [`LENS_OVERRIDES`]; the lens pass applies it
//!   to the camera's lens. Without a publisher the lens uses its own values.
//!
//! How others do it: Unreal blends depth of field, vignette and chromatic
//! aberration in post-process volumes and keeps the physical camera settings
//! on the camera component; this keeps that split, with the lens owning both
//! halves of its data.

pub mod flare;
pub mod overrides;

pub use flare::*;
pub use overrides::*;
