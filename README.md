# Plugin_Lens

A camera's physically based lens: its component, presets, rows and the renderer features that resolve the image through it and draw its flare.

`pulsar.lens` ("Camera Lens") is a plugin of
[Pulsar-Native](https://github.com/Far-Beyond-Pulsar/Pulsar-Native), the engine and editor that renders with
[Helio](https://github.com/Far-Beyond-Pulsar/Helio). Pulsar-Native vendors this repository as a git submodule
at `plugins/vendor/lens`.

## What it owns

Every object kind is a plugin. A plugin owns its SceneDB components (their
data and scriptable methods; the editor calls the same methods scripts do),
the contracts other kinds read (its `api` crate: graph keys, row ABIs, WGSL
snippets), the Helio feature that renders it (passes, scene derivations,
settings), its file formats and its editor UI. Helio knows no object kind,
and other kinds meet this one only through frame contracts, extension points,
draw phases and api crates.

| Crate | Path | What |
|---|---|---|
| `lens_component` | `component/` | Camera lenses (pulsar.lens): the camera lens component, its rows and presets, and the registrations of the plugin's Helio features (lens, flare) |
| `lens_plugin` | `editor/` | Camera Lens (pulsar.lens): the editor plugin -- its metadata, over the plugin's component crate |
| `lens-examples` | `examples/` | The lens plugin's standalone hosts on Helio: the lens showcase and the lens flare demo |
| `helio-pass-flare` | `feature/flare/` | Helio render pass: image-based scene-linear lens response (ghosts, halo, glare, streaks) |
| `helio-pass-lens` | `feature/lens/` | Helio render pass: physically based screen-space lens (thin-lens focus field and defocus) |
| `helio-lens-optics` | `feature/optics/` | Pure-CPU real lens prescription tracer and lookup-table baker for the Helio lens pass |

- **Runtime module** (what a game built for a project links): `lens_component`.
- **Editor module** (what the editor links in addition): `lens_plugin`.
- **Requires**: nothing. A project enables or disables the plugin in its
  `Pulsar.toml` (`[plugins]`); disabled, the kind is absent and the frame
  draws without it.
- **Reads other kinds through**: `helio-light-api` (lights).
- **Its standalone hosts also compose**: `helio-pass-gi` (gi); `helio-light-api, helio-light-host` (lights); `helio-mats` (materials); `helio-mesh-api, helio-mesh-host, helio-pass-forward-lit` (meshes); `helio-pass-post-volumes` (post); `helio-pass-sky` (sky) (demos and GPU tests
  draw a whole scene; the plugin itself does not depend on these).

## Building and testing

Like [Plugin_Physics](https://github.com/Far-Beyond-Pulsar/Plugin_Physics), this plugin's crates are members
of Pulsar-Native's workspace: their dependencies are `workspace = true`, and
Helio comes from Pulsar-Native's `crates/renderer/helio` submodule. It builds
only inside a Pulsar-Native checkout:

```bash
git clone --recursive https://github.com/Far-Beyond-Pulsar/Pulsar-Native
cd Pulsar-Native
cargo check -p lens_component -p lens_plugin -p lens-examples -p helio-pass-flare -p helio-pass-lens -p helio-lens-optics --tests
cargo test -p lens_component -p helio-pass-flare -p helio-pass-lens -p helio-lens-optics
```

Suites that render real frames need a GPU; CI leaves them out by name
(Pulsar-Native's `.config/nextest.toml`, `ci` profile). This repository's CI
checks the plugin inside a Pulsar-Native checkout: rustfmt, clippy and the
`ci` profile's tests.

A change that spans this plugin and Pulsar-Native (or Helio) merges
dependency-first: merge here, re-pin `plugins/vendor/lens` in Pulsar-Native to
this repository's `main`, then merge the Pulsar-Native PR (Pulsar-Native's
`.agents/CRATES.md`, "Submodule pins").

## Architecture

- [Every object kind is a plugin](https://github.com/Far-Beyond-Pulsar/Helio/blob/main/docs/architecture/plugin-migration.md): the
  rule, what Helio is, the plugin table and the "As built" notes of each
  phase.
- [The layered rewrite](https://github.com/Far-Beyond-Pulsar/Helio/blob/main/docs/architecture/layered-rewrite.md): layers, buffer
  contracts, extension points and why the renderer has this shape.
- [The `helio-architecture` skill](https://github.com/Far-Beyond-Pulsar/Helio/blob/main/.agents/skills/helio-architecture/SKILL.md): required reading for any change
  here (see `AGENTS.md`).
- Pulsar-Native's `.agents/PLUGIN_SYSTEM.md` and `.agents/CRATES.md`: plugin
  descriptors, the installed set and the submodule pins.

History before 2026-11-02 (when the plugin moved into Pulsar-Native) is in
Helio's repository; this repository's history starts with that move
(`git subtree split` of `plugins/vendor/lens`).

## License

MIT, as the rest of Pulsar (`LICENSE.md`).
