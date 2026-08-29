# Arcademic Rust

`arcademic-rust` is a configuration-driven, statically generated Dioxus
reimplementation of `../arcademic-nuxt`.

The migration is being delivered in independently accepted and committed
phases. The authoritative documents are:

- [Source audit](docs/source-audit.md)
- [Framework decision](docs/architecture/adr-001-dioxus.md)
- [Delivery and acceptance plan](docs/migration-plan.md)
- [Configuration guide](docs/configuration.md)
- [Deployment guide](docs/deployment.md)
- [Final completion audit](docs/acceptance/final-audit.md)

The application implementation starts in phase 1. The target output is a
pre-rendered static site that can be hosted from a GitHub Pages project path
without a runtime server.

## Build commands

```bash
cargo test --all-features
scripts/build-pages.sh arcademic-rust release
```

The second command writes the deployable artifact to `dist/public`. The base
path argument is omitted for root-domain hosting.

## Browser acceptance

Install the pinned browser-test dependencies and browsers once:

```bash
npm ci
npx playwright install chromium firefox webkit
```

After serving the repository-path release at
`http://127.0.0.1:3200/arcademic-rust/`, run behavior, accessibility, and
no-JavaScript checks in all three engines:

```bash
npm run test:e2e
```

The source-to-Rust visual gate additionally requires the generated Nuxt source
at `http://127.0.0.1:3300/`:

```bash
SOURCE_BASE_URL=http://127.0.0.1:3300/ npm run test:visual
```

Playwright retains traces, screenshots, and direct source/Rust/diff crops in
`test-results` when a test fails. Remote image pixels and the random blinking
grid are stabilized before comparison.
