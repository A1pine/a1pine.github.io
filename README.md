# Arcademic Rust

`arcademic-rust` is a configuration-driven, statically generated Dioxus
reimplementation of `../arcademic-nuxt`.

The migration is being delivered in independently accepted and committed
phases. The authoritative documents are:

- [Source audit](docs/source-audit.md)
- [Framework decision](docs/architecture/adr-001-dioxus.md)
- [Delivery and acceptance plan](docs/migration-plan.md)
- [Configuration guide](docs/configuration.md)

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
