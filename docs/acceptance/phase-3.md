# Phase 3 acceptance record

Accepted on 2026-08-29 against the release SSG artifact at
`http://127.0.0.1:3200/arcademic-rust/`.

## Automated gates

Formatting, Clippy with warnings denied, native tests, the dedicated WASM web
check, and the release Pages build passed. The Rust suite contains 26 tests;
phase 3 adds publication-count normalization and tag palette/fallback cases.

The static `index.html` contains all three news items, both summary values,
five yearly statistics, and all three publication records before JavaScript.

## Browser evidence

Playwright scrolled each reveal target into view and waited on browser state,
not timeouts. Observed results:

- Three News cards entered once through IntersectionObserver.
- Product, Research, and Keynote resolved to their configured blue, purple,
  and amber palettes.
- Two summary cards, five bars, and three publication cards rendered.
- Final bar scale values were exactly 0.208333, 0.333333, 0.5, 0.75, and 1.
- Hovering 2024 displayed `24 Papers` at opacity 1.
- Mobile Research stats used one 358 px column and had no page overflow.
- Dark Product palette resolved to `rgba(30, 58, 138, 0.4)` with
  `rgb(191, 219, 254)` text.
- JavaScript-disabled content remained visible at opacity 1.
- Console errors: 0.

The reveal observer adds its pending class only after hydration. This avoids
hiding content in no-JavaScript or observer-unsupported clients while
preserving viewport-triggered motion in capable browsers.

## Screenshots

- `phase-3-news-desktop.png`
- `phase-3-stats-desktop.png`
- `phase-3-stats-mobile.png`

Publication action controls, highlighted authors, final card hover details,
and the remaining sections are completed in phase 4.

