# Performance Architecture

The site keeps all entrance, reveal, hover, background, heatmap, scroll, and
theme-transition effects. Performance work targets transfer size, parsing,
hydration, rendering isolation, and unnecessary hidden work instead.

## Measured Baseline

The same local release server and Lighthouse configuration produced:

| Metric | Before | Optimized |
|---|---:|---:|
| Lighthouse Performance | 72 | 78 |
| First Contentful Paint | 2.25 s | 1.40 s |
| Largest Contentful Paint | 11.44 s | 5.95 s |
| Total Blocking Time | 16.5 ms | 0.5 ms |
| Cumulative Layout Shift | 0 | 0 |
| Main-thread work | 1,603 ms | 736 ms |
| Uncompressed local transfer | 2.09 MB | 999 KB |

Local serving uses HTTP/1 without transfer compression, so production
Cloudflare timings are expected to be better. Comparisons are useful because
both measurements use the same environment.

The optimized production deployment at `https://blog.xuningtan.com/` scores
98 for Performance and 100 for Accessibility, Best Practices, and SEO. Its
mobile Lighthouse run reports FCP 1.99 s, LCP 1.99 s, TBT 3 ms, CLS 0, and
approximately 412 KB total transfer.

## Implemented Controls

- Release builds use size optimization, LTO, one codegen unit, panic abort,
  and stripped symbols.
- Build-time TOML-to-JSON generation keeps TOML, URL/IDNA, and validation code
  out of the browser WASM.
- Loading and failure states use one animated heatmap skeleton; the 371 live
  cells and their staggered animations are created only after data arrives.
- Below-fold sections use `content-visibility` and an intrinsic-size fallback.
- The portrait is a 576 x 576, approximately 31 KB JPEG with fixed geometry.
- The live VibeUsage SVG is below the fold, has fixed geometry, and loads lazily.
- The generated HTML preloads the content-hashed WASM and preconnects to the
  contribution API.
- Scroll progress updates a CSS variable every animation frame, while Dioxus
  state changes only when the scroll-to-top threshold is crossed.
- The hidden blinking grid pauses in dark mode and while the document is not
  visible; its light-mode animation remains unchanged.
- Artifact verification enforces content hashes and byte budgets.

## Release Budgets

| Resource | Maximum raw size |
|---|---:|
| `index.html` | 40 KB |
| WASM | 900 KB |
| JavaScript | 70 KB |
| CSS | 40 KB |
| LCP JPEG | 60 KB |

The verifier also requires every asset filename to include a Dioxus content
hash and requires the WASM preload in generated HTML.

## Edge Configuration

GitHub Pages does not honor a repository `_headers` file. Configure Cloudflare
for `/assets/*` with `Cache Everything`, one-year browser and edge TTLs, and:

```text
Cache-Control: public, max-age=31536000, immutable
```

Keep HTML on a short TTL and purge it after deployment. Explicitly enable
Brotli for WASM, JavaScript, CSS, SVG, and HTML. The current optimized WASM is
about 323 KB with gzip and 252 KB with Brotli.
