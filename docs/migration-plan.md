# Migration, test, and acceptance plan

## Delivery rules

Each phase is accepted independently and committed before the next phase
starts. A phase is complete only when all listed commands and browser checks
pass from a clean checkout. Commit subjects use `phase N:`.

No acceptance claim may rely only on compilation. Generated HTML, browser
runtime behavior, screenshots, and the deployed subpath are authoritative for
their respective requirements.

## Phase 0: Audit and architecture

Tasks:

- Inspect the Nuxt page tree, components, composables, styles, data, config,
  generated output, and working-tree changes.
- Record all rendered modules, content, responsive rules, state transitions,
  animations, hover effects, and known source defects.
- Evaluate all seven requested Rust frameworks using weighted requirements.
- Select and document the framework, trade-offs, risks, and revisit triggers.
- Capture desktop/light, desktop/dark, mobile/light, and mobile-menu baselines.
- Define the phased test and acceptance gates below.

Tests and acceptance:

- All seven candidates appear in ADR-001.
- Every rendered section and global control appears in the source audit.
- The config audit includes all visible information, URLs, behavior values,
  animation timing, and accessibility labels.
- Baseline images have non-zero dimensions and can be decoded.
- `git diff --check` passes.

Commit: `phase 0: document source audit and framework decision`

## Phase 1: Dioxus SSG foundation and configuration contract

Tasks:

- Initialize a pinned Dioxus 0.7 fullstack project with web/server features.
- Add `Dioxus.toml`, Rust toolchain metadata, reproducible local commands, and
  a GitHub Pages workflow.
- Port and extend `site.toml` into typed Serde structures with `deny_unknown_fields`.
- Implement validation for required strings, unique ids/anchors, URL schemes,
  working-hour ranges, activity dimensions, stat ranges, animation values,
  template placeholders, and internal navigation targets.
- Embed the same validated config in server and web builds.
- Configure `/` as an SSG route and make the repository base path injectable.
- Add a minimal semantic shell proving pre-render and hydration.

Unit tests:

- The production TOML parses and validates.
- Unknown keys and missing required fields fail.
- Duplicate section/item ids fail.
- Invalid internal anchors, URL schemes, hour ranges, grid dimensions, and
  animation durations fail with field-specific errors.
- Footer placeholder substitution and base-path joining are deterministic.

Integration and acceptance:

- `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`,
  and `cargo test --all-features` pass.
- `scripts/build-pages.sh arcademic-rust release` succeeds using Dioxus SSG.
- Generated `index.html` contains configured name, biography, section labels,
  publication titles, course titles, and activity labels before JavaScript.
- The generated artifact contains no absolute root asset URL when built with
  a repository base path.
- Chromium loads the artifact with no console/page errors and hydrates a test
  theme toggle without replacing the document.

Commit: `phase 1: establish dioxus ssg and configuration model`

## Phase 2: Global design, navigation, Hero, and background

Tasks:

- Port reset, typography, colors, glass surfaces, responsive containers, dark
  theme, focus styles, and reduced-motion rules to explicit CSS.
- Implement fixed Navbar, desktop links, mobile menu, theme persistence,
  theme icon switch, CV effect, and reading-progress line.
- Implement Hero image/glow/status, contact card, biography, interests,
  social links, and functional `#about` anchor.
- Implement pointer glow, coarse-pointer centering, blinking grid, visibility
  pause/resume, resize handling, and cleanup.
- Implement scroll state with one passive listener and animation-frame
  coalescing.

Unit/component tests:

- Work-hours status at start, end, inside, and outside configured hours.
- Social icon fallback and labels.
- Theme preference resolution and storage value parsing.
- Scroll progress clamps to `[0, 1]`, including zero scroll distance.
- Blinking cell count remains within configured bounds and coordinates remain
  inside computed grid dimensions using a seeded random source.

Browser acceptance at 1440 x 1000 and 390 x 844:

- Navbar geometry, Hero stacking, image size, text, links, and anchors match.
- Theme toggles and survives reload.
- Mobile menu opens, exposes all configured links, closes by its control and
  after navigation, and returns focus appropriately.
- Progress reaches approximately 0%, 50%, and 100% at corresponding scroll.
- Fine-pointer glow follows pointer without layout changes; coarse pointer is
  centered; hidden documents stop adding cells.
- No horizontal overflow except explicitly scrollable components.

Commit: `phase 2: recreate navigation hero and background`

## Phase 3: News and research output

Tasks:

- Implement News heading, timeline, dots, tag variants, cards, responsive
  typography, and staggered viewport entry.
- Implement publication heading/subtitle, totals, citation card, normalized
  bar chart, hover tooltip, and bar-entry animation.
- Unit-test tag fallback and bar normalization with empty, zero, and ordinary
  data.

Acceptance:

- Configured news order/content/tags are present in generated HTML.
- Timeline offsets and card widths match at desktop/mobile breakpoints.
- Five bars have correct relative heights and year/count tooltips.
- Entry animations run once; reduced-motion mode presents final state without
  animation.

Commit: `phase 3: recreate news and research statistics`

## Phase 4: Publications, teaching, activity, footer, scroll control

Tasks:

- Implement all publication cards, responsive images/srcsets, authors,
  highlighted author, tags, and PDF/Code actions.
- Implement teaching grid and fixed-height title behavior.
- Implement the deterministic 52 x 7 activity grid, month/day labels, legend,
  horizontal mobile scrolling, cell titles, stagger, and hover scale.
- Implement Footer template substitution and ScrollToTop behavior.

Unit/component tests:

- Publication action renders link when configured and disabled control when
  absent; external link security attributes are correct.
- Author separator/highlight output is correct for one and many authors.
- Teaching order and configured semester prefix are preserved.
- Activity level generator matches golden cases and produces only levels 0-4.
- Exactly `weeks * days` cells render with unique ids.
- Footer substitutes all required placeholders and rejects unknown ones.
- Scroll button threshold uses strict `>` parity and requests smooth scroll.

Browser acceptance:

- Three publication cards, four teaching cards, and 364 heatmap cells exist.
- Publication images preserve aspect and do not shift layout when blocked.
- Heatmap is horizontally scrollable only where needed and hovered cells do
  not clip or resize layout.
- Scroll control is hidden at 300 px, visible above 300 px, and reaches top.

Commit: `phase 4: complete content sections and page controls`

## Phase 5: Configuration completeness, accessibility, and SEO

Tasks:

- Audit Rust source for profile-specific strings and migrate any remaining
  information to TOML.
- Add metadata, canonical/social tags, favicon, image dimensions, lazy/eager
  loading parity, accessible names, landmark structure, and skip navigation.
- Add config examples and authoring documentation.
- Enforce keyboard focus, reduced motion, contrast, and no-JavaScript content.

Tests and acceptance:

- A second fixture with entirely different identity/content produces a page
  with none of the Tony Stark defaults.
- Static-source scan allows no default profile strings outside fixtures/config.
- Axe reports no serious or critical violations in both themes and viewports.
- Keyboard-only navigation can operate theme, menu, links, actions, heatmap
  scroll region, and scroll-to-top.
- Lighthouse targets: Accessibility >= 95, SEO >= 95, Best Practices >= 90.
- With JavaScript blocked, all content and valid outbound links remain visible.

Commit: `phase 5: harden configuration accessibility and metadata`

## Phase 6: End-to-end and visual parity gate

Tasks:

- Add isolated Playwright tests with traces and screenshots retained on
  failure; use role/text/label locators.
- Capture source and Rust section screenshots only after activating viewport
  animations; block or fixture remote images for deterministic runs.
- Add pixel comparison for desktop/light, desktop/dark, mobile/light,
  mobile/dark, mobile menu, CV hover, publication hover, chart tooltip,
  heatmap hover, and scroll-to-top states.
- Exercise Chromium, Firefox, and WebKit for behavior; run pixel comparison on
  the pinned Chromium only.

Acceptance:

- Every module and interaction in the source-audit matrix has a passing E2E
  assertion.
- No test uses arbitrary timeout sleeps.
- Layout landmarks differ by at most 2 CSS px; text/container overlap count is
  zero; no unintended horizontal page overflow exists.
- Pixel difference is <= 0.5% per stable clipped section after masking remote
  image pixels and intentional random blinking cells.
- Browser console and page-error arrays are empty in every project.

Commit: `phase 6: enforce end to end and visual parity`

## Phase 7: Production and deployment audit

Tasks:

- Run release SSG with root and repository-path configurations.
- Serve both artifacts locally with history/path behavior equivalent to
  GitHub Pages and run the complete acceptance suite.
- Verify workflow permissions, caching, artifact upload, Pages deployment,
  custom 404 behavior, and generated file inventory.
- Write configuration and deployment documentation.
- Perform a requirement-by-requirement completion audit against this plan and
  the source-audit matrix.

Acceptance:

- Fresh CI-equivalent commands produce the same artifact from a clean clone.
- Root and `/arcademic-rust/` previews load all CSS, WASM, JS, icons, and
  images with zero failed required requests.
- `index.html` is pre-rendered; no server executable is present in the Pages
  artifact; `.nojekyll` exists.
- All unit, clippy, formatting, E2E, accessibility, Lighthouse, and visual
  gates pass against the release artifact.
- The final audit maps every explicit objective and phase criterion to direct
  evidence and reports no missing or indirect proof.

Commit: `phase 7: finalize github pages delivery`
