# Phase 2 acceptance record

Accepted on 2026-08-29 against the release SSG artifact at
`http://127.0.0.1:3200/arcademic-rust/`.

## Design direction

The source direction is retained: a restrained academic profile with an Arc
Reactor-inspired cyan glow and glass surfaces. The frontend-design DFII score
is 13: impact 4, context fit 5, feasibility 5, performance safety 4, and
consistency risk 5. No redesign was introduced because source parity is the
governing requirement.

## Automated gates

The following passed:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo check --target wasm32-unknown-unknown --no-default-features --features web
scripts/build-pages.sh arcademic-rust release
```

There are 22 Rust tests. Phase 2 adds direct cases for stored/system theme
resolution, work-hour boundaries across timezones, Unsplash srcset rewriting,
scroll progress clamping, generated grid coordinates, and random target cell
bounds.

## Browser evidence

Playwright exercised the release artifact in isolated desktop and mobile
contexts. Remote profile/publication images were replaced only for behavioral
checks; the committed screenshots use the real configured images.

Measured desktop results at 1440 x 1000:

- Hero height: 745 px; source baseline: 742 px.
- Final portrait width: 288 px.
- Navbar height including border: 65 px.
- Document width: 1440 px; viewport width: 1440 px.
- Scroll progress after scrolling 1000 px: 0.2616.
- Portrait filter changed from `grayscale(1)` to `grayscale(0)` on hover.
- Pointer glow moved through a requestAnimationFrame-coalesced transform.

Measured mobile results at 390 x 844:

- Final portrait width: 256 px.
- Six configured navigation links appeared in the open menu.
- Menu navigation reached `#about` and closed the menu.
- Coarse-pointer glow centered at 195 x 422 px.
- Document and viewport widths were both 390 px.

State and runtime results:

- Theme resolved to light under a light system preference.
- Theme toggle persisted `dark` in `arcademic-color-mode` and survived reload.
- Scroll progress stayed within `[0, 1]`.
- Blinking grid reached a configured stable count of 6 (allowed 6 through 10).
- Console errors: 0; failed local requests: 0.
- Reduced-motion CSS resolves all animations and transitions to their final
  state in 1 ms.

## Screenshots

- `phase-2-desktop-light.png`
- `phase-2-mobile-light.png`
- `phase-2-mobile-menu.png`

The screenshots cover the phase 2 surface only. News, Publications, Teaching,
Activity, Footer, and ScrollToTop retain semantic placeholder styling until
their own implementation phases.

