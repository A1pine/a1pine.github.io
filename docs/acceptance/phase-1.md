# Phase 1 acceptance record

Accepted on 2026-08-29 by direct local execution.

## Toolchain

- Rust 1.91.1
- Dioxus application 0.7.3 with a locked dependency graph
- Dioxus CLI 0.7.3
- Chromium from Playwright 1.62.1

## Static generation evidence

Command:

```bash
DIOXUS_CLI=/Users/tanxuning/.cargo/bin/dx \
  scripts/build-pages.sh arcademic-rust release
```

Observed evidence:

- Dioxus reported `Rendering / for SSG` and `SSG complete`.
- `dist/public/index.html` is 12,216 bytes and contains, before JavaScript,
  the complete biography, News, Research, Teaching, and Activity content.
- All generated local CSS, favicon, JavaScript, and WASM references use the
  `/arcademic-rust/` project prefix.
- `dist/public/.nojekyll` exists.
- The deployable `dist/public` directory contains no server executable.

## Rust gates

These commands passed:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

The configuration suite covers production parsing, unknown and missing
fields, duplicate item and section ids, unknown anchors, invalid URL schemes,
working-hour and Activity ranges, animation duration, footer placeholders,
base-path joining, and deterministic theme transition.

## Browser gates

The release artifact was served at
`http://127.0.0.1:3200/arcademic-rust/`. External Unsplash requests were
fulfilled with deterministic one-pixel fixtures; local requests were not
mocked.

Observed evidence:

- Page title: `Tony Stark Academic Profile`.
- Five main sections and seven content articles were present.
- The theme control changed `data-theme` from `system` to `dark`.
- The original server-rendered shell node remained connected after hydration.
- JavaScript-disabled Chromium still exposed the `Tony Stark` H1.
- Browser console errors: 0.
- Failed local requests: 0.

Visual parity is intentionally not claimed by this phase. The minimal semantic
shell proves the configuration, SSG, base-path, and hydration architecture;
the source design is implemented and accepted in phases 2 through 6.

