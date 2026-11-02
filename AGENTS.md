# Nongle

Multi-layered xylophone rompler / sampler. Pure Rust, minimal deps.

## Layout

- `engine/` — sample library, velocity layer state machine.
- `vst/` — nih-plug CLAP/VST3 plugin, iced GUI.
- `standalone/` — CLI host for dev.
- `nih_plug_iced/` — local fork of the iced adapter.
- `xtask/` — bundling helpers.

## Build

Bundle the plugin with `cargo xtask bundle vst` (or `just build_debug`,
`just build_release`). Full install-and-relaunch flows live in the `justfile`.
`cargo build` alone does not produce a loadable plugin bundle.

## Conventions

- Comments in English only.
- Sample libraries are arbitrary folders, loaded via the UI's "Load Library" button. No built-in library list.

## Testing

No automated tests. Plugin code needs a running host/audio context; testing
strategy is a separate decision. Do not add tests.
