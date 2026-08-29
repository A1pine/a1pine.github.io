# Nuxt source audit

## Audit scope

The source inspected is `../arcademic-nuxt` on branch
`perf-runtime-optimization`, including committed revision `d882eb8` and the
working tree present on 2026-08-29. The source working tree changed during the
audit, so both states below are material:

- The last static artifact, `.output/public/index.html`, was generated at
  2026-08-29 10:32:24 +08:00 and renders successfully.
- The source at 10:48-10:51 introduced build-time `__SITE_CONFIG__` injection,
  custom motion, lazy hydration, shared scroll state, and image `srcset`
  changes. At audit time its dev SSR returned 500 because
  `__SITE_CONFIG__ is not defined` in the Nitro execution path.

The Rust rewrite will follow the latest source intent while using the last
successful static artifact as the visual reference. The adjacent repository
is read-only for this migration.

## Page composition

The rendered page order is:

1. Fixed global background
2. Fixed navigation bar and reading-progress line
3. Hero/profile (`#home`)
4. News timeline (`#news`)
5. Research summary and publications (`#publications`)
6. Teaching cards (`#teaching`)
7. Contribution heatmap (`#activity`)
8. Footer
9. Teleported scroll-to-top control

`Bio.vue` and its JSON/composables still exist, but the committed migration
notes explicitly removed it from `pages/index.vue` because Hero already owns
the same biography. It is dead source, not a second rendered section. The
Rust component inventory will preserve the information once in Hero rather
than duplicate the dead component.

## Rendered geometry baseline

At a 1440 x 1000 Chromium viewport, the last successful static artifact
reported a 4164 px document body with these section bounds:

| Section | Top | Height |
|---|---:|---:|
| `home` | 0 | 742 |
| `news` | 742 | 782 |
| `publications` | 1524 | 1625 |
| `teaching` | 3149 | 478 |
| `activity` | 3627 | 452 |

The relevant responsive breakpoints are 640 px, 768 px, and 1024 px. Desktop
uses a two-column Hero, horizontal navigation, a three-column research stats
area, horizontal publication cards, and four teaching columns. Mobile centers
and stacks Hero content, replaces navigation with a dropdown, stacks research
cards, and horizontally scrolls the 800 px contribution grid.

## Module and behavior inventory

| Module | Content | State and behavior | Motion/effects |
|---|---|---|---|
| Background | colors and glow settings | tracks fine-pointer position with `requestAnimationFrame`; centers on coarse pointers; light-only blinking grid; pauses when hidden | 900 px radial glow, 60 px blur, screen blend, random grid cells, 3 s blink |
| Navbar | brand, six links, CV label/URL, ARIA labels | fixed; light/dark toggle persisted under `nuxt-color-mode`; mobile open/close; shared scroll progress | 300 ms color/underline transitions, CV particle/glow variants, 3 px cyan progress line |
| Hero | name, role, location, company, biography, five interests, three social links, image | work-hours indicator calculated from UTC offset and configured hours | image scale/fade entry; content slide/fade; pulsing cyan image glow; grayscale-to-color hover; card/link hover |
| News | heading and three tagged items | tag style mapping with fallback | spring-like heading and staggered card entries; timeline-dot and card hover |
| Publications | headings, totals, five yearly stats, three publications | max-normalized bars; author highlighting; configurable images/tags; currently inert PDF/Code buttons | spring-like section/stat/card entries; bar growth; tooltip; image zoom; card/button/tag hover |
| Teaching | heading and four courses | responsive 1/2/4-column layout | heading entry; staggered card entry; lift and border/title hover |
| Activity | labels, dimensions, deterministic generated levels | 52 x 7 cells; level formula `(w*17 + d*31 + w*d*7 + 13) % 100`; horizontal mobile scroll | staggered 6 ms cell entry and 1.3 hover scale |
| Scroll to top | tooltip and threshold | visible above 300 px; smooth scroll to zero | opacity, scale, and shadow transitions |
| Footer | owner, services, template | current year substituted at render time | theme color transition |

## Configuration audit

`config/site.toml` already owns the main profile information, but 1:1
configurability requires extending it to cover every remaining user-facing or
data-bearing value:

- Site title, description, language, canonical URL, favicon, social preview,
  default theme, theme storage key, and base path.
- Every navigation label, target, control label, and whether CV is shown.
- Hero content, image attributes, work timezone/hours, interest list, and
  social platform/icon/link data.
- Every section heading, subtitle, item, tag style key, statistic, tooltip,
  action label, action URL, and author-highlight rule.
- Activity dimensions, month/day labels, legend labels/colors, deterministic
  seed parameters, and tooltip template.
- Footer template and dynamic year behavior.
- Background dimensions/colors/cell counts/interval and scroll threshold.
- Animation enablement, reduced-motion behavior, durations, delays, stagger,
  easing/spring approximation, and hover scaling values.

Layout-specific CSS values may have defaults in CSS, but every value that
changes information, enabled behavior, labels, timing, data, links, media, or
theme identity belongs in TOML.

## Parity defects found in the source

These are recorded rather than silently copied:

1. Navbar `About` targets `#about`, but no rendered element owns that id.
   The Rust site will add the anchor to Hero's biography region without
   changing layout.
2. Publication PDF and Code controls are buttons with no action. The Rust
   configuration will accept URLs; absent URLs render disabled controls with
   the same appearance and correct semantics.
3. `Bio.vue`, JSON fixtures, Supabase migrations, and old data composables are
   not used by the current rendered page.
4. Current Nuxt dev SSR is broken by the incomplete config injection change.
   This is not a prerequisite or a file to repair in the Rust repository.
5. Full-page screenshots can omit offscreen `content-visibility: auto`
   sections until a browser scrolls them into view. Visual tests must activate
   sections before comparison.

## Baseline artifacts

The `docs/baseline` images are evidence from the last successful static Nuxt
artifact, not hand-authored design mockups:

- `nuxt-desktop-light.png`: initial light rendering at 1440 x 1000.
- `nuxt-desktop-dark.png`: initial dark rendering at 1440 x 1000.
- `nuxt-mobile-light.png`: initial light rendering at 390 x 844.
- `nuxt-mobile-menu.png`: open mobile navigation at 390 x 844.

The initial full-page images intentionally demonstrate the source's
content-visibility behavior. Final visual acceptance will additionally use
section-level screenshots after scrolling each section into view.

