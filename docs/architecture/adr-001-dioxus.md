# ADR-001: Use Dioxus 0.7 for the static academic site

## Status

Accepted on 2026-08-29.

## Context

The source is a single-page academic profile built with Nuxt 3. It needs a
Rust rewrite with 1:1 semantic structure, responsive layout, theme behavior,
DOM animations, pointer effects, and static deployment to GitHub Pages. All
site content and user-facing labels must come from configuration.

This is a web document, not a native GUI. Search engines and no-JavaScript
clients must receive the complete content in the initial HTML. The client
bundle is only responsible for hydration and interaction.

The candidates requested for evaluation are Yew, Iced, egui, Dioxus, Leptos,
Makepad, and Sycamore.

## Decision criteria

Scores are 1 (poor fit) through 5 (excellent fit). Weighted total is out of
5.00.

| Criterion | Weight | Yew | Iced | egui | Dioxus | Leptos | Makepad | Sycamore |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Semantic HTML and SEO | 25% | 5 | 1 | 1 | 5 | 5 | 1 | 5 |
| Exact CSS/DOM fidelity | 20% | 5 | 1 | 1 | 5 | 5 | 1 | 5 |
| Browser interaction model | 15% | 5 | 2 | 2 | 5 | 5 | 2 | 5 |
| Built-in static generation | 15% | 2 | 1 | 1 | 5 | 4 | 1 | 3 |
| Stable ecosystem/tooling | 10% | 5 | 4 | 5 | 4 | 4 | 2 | 3 |
| Testability | 10% | 4 | 3 | 3 | 5 | 5 | 2 | 3 |
| Runtime and bundle fit | 5% | 4 | 2 | 2 | 4 | 5 | 2 | 5 |
| **Weighted total** | **100%** | **4.35** | **1.65** | **1.75** | **4.85** | **4.75** | **1.45** | **4.30** |

## Candidate assessment

### Dioxus

Dioxus 0.7.3 is selected. Its stable fullstack toolchain supports static site
generation directly: `dx bundle --web --ssg` runs the application, obtains a
static route list, pre-renders HTML, and emits a `public` directory containing
the HTML, assets, JavaScript, and WASM. The official documentation explicitly
identifies GitHub Pages as a target for this output.

RSX produces normal HTML, so the existing Tailwind-derived design can be
ported to explicit CSS without canvas rendering or accessibility shims.
Dioxus signals and effects cover the small client state surface without a
separate JavaScript application.

### Leptos

Leptos 0.8.8 is the closest alternative. It has first-class SSR/hydration and
static-route primitives, excellent fine-grained reactivity, and documented
GitHub Pages deployment for CSR applications. It scores slightly lower here
because producing a fully pre-rendered GitHub Pages artifact requires more
assembly across cargo-leptos/static-route/subpath concerns than the dedicated
Dioxus SSG bundle command.

### Yew

Yew 0.22 is mature and gives precise DOM control. Its normal GitHub Pages path
is a client-rendered Trunk application. SSR exists, but a static-generation
pipeline and hydration artifact assembly would be project-owned plumbing.

### Sycamore

Sycamore 0.9.2 is lightweight and supports SSR/hydration. Its ecosystem,
testing surface, and turnkey static-generation/deployment workflow are less
complete than Dioxus or Leptos for this delivery.

### Iced, egui, and Makepad

These are immediate/native GUI or custom-renderer frameworks. Their web
targets are appropriate for application canvases, editors, games, or shared
native/web UIs. Recreating a content site with them would sacrifice native
document semantics, CSS fidelity, text selection, SEO, accessibility, and
static HTML. They are rejected regardless of their native rendering quality.

## Decision

Use these architectural boundaries:

- Dioxus 0.7.3, pinned to the stable minor line.
- One fullstack crate with `web` and `server` feature sets.
- Dioxus SSG for `/`; no production server is deployed.
- `config/site.toml` is the single content and behavior source of truth.
- Serde models parse and validate configuration before any build succeeds.
- Components receive typed configuration and contain no profile-specific
  strings, URLs, publication data, course data, or user-facing labels.
- CSS owns visual effects and responsive layout; Rust owns state and event
  wiring. Browser APIs are used only where CSS cannot express behavior.
- Static assets and generated URLs must work at both `/` and a GitHub Pages
  project base path.

## Trade-offs accepted

- Dioxus SSG uses the fullstack build path at build time even though no server
  is shipped. CI is therefore heavier than a pure CSR build.
- The Dioxus CLI becomes part of the reproducible toolchain.
- Exact parity requires explicit CSS rather than relying on a Rust widget
  library. This is intentional because the source is itself a CSS/DOM site.
- Remote Unsplash images remain configurable external dependencies by
  default. Tests will intercept them, and documentation will support local
  replacements for deterministic deployments.

## Consequences

Positive outcomes:

- Initial HTML contains all configured information.
- GitHub Pages receives only static files.
- DOM semantics, keyboard behavior, responsive rules, and CSS animation can
  match the source directly.
- Content validation and derived-data behavior are unit-testable in Rust.

Risks and mitigations:

- Dioxus CLI or SSG changes: pin versions and run the real SSG command in CI.
- Hydration mismatch: use one component tree and the same embedded config for
  server rendering and the web build; fail E2E tests on console errors.
- Project subpath failures: validate asset requests from a non-root base-path
  preview before deployment.
- Visual drift: compare fixed desktop/mobile screenshots after activating all
  viewport animations.

## Revisit triggers

Reconsider Leptos only if Dioxus 0.7 cannot satisfy one of these proven gates:

1. Complete profile content is absent from generated `index.html`.
2. Hydration cannot preserve the server-rendered DOM without errors.
3. The generated bundle cannot be hosted under a repository subpath.
4. Required semantic or browser interactions need a custom renderer.

