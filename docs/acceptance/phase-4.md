# Phase 4 acceptance record

Accepted on 2026-08-29 against the release SSG artifact at
`http://127.0.0.1:3200/arcademic-rust/`.

## Automated gates

Formatting, Clippy with warnings denied, native tests, the WASM web check,
release SSG, and generated-content checks passed. The Rust suite contains 31
tests. Phase 4 adds deterministic Activity golden cases, complete grid size and
range checks, publication action states, external-link classification, and the
strict ScrollToTop threshold.

## Browser evidence

Playwright observed:

- Three publication cards with responsive images.
- Three highlighted `T. Stark` author spans.
- Six disabled PDF/Code buttons because the default URLs are empty.
- External action configuration maps to `_blank` with
  `noopener noreferrer`; internal configuration remains same-context.
- Four teaching cards in four 286 px desktop columns and one 358 px mobile
  column.
- Exactly 364 uniquely delayed heatmap cells with levels 0 through 4.
- Heatmap hover transform: `matrix(1.3, 0, 0, 1.3, 0, 0)`.
- No page overflow at 1440 px or 390 px.
- Mobile heatmap viewport: 308 px; scroll width: 864 px.
- ScrollToTop hidden at exactly 300 px and visible at 301 px.
- Clicking ScrollToTop with configured smooth behavior reached `scrollY = 0`.
- Footer text matched the configured template and build year.
- Console errors: 0.

The Navbar reading-progress line and ScrollToTop share one passive,
requestAnimationFrame-coalesced scroll tracker. Heatmap entry uses `backwards`
fill so the completed animation releases `transform` for hover interaction.

## Screenshots

- `phase-4-publication.png`
- `phase-4-teaching.png`
- `phase-4-activity.png`

