import { expect, test, type Browser, type Locator, type Page, type TestInfo } from '@playwright/test'
import { writeFileSync } from 'node:fs'
import pixelmatch from 'pixelmatch'
import { PNG } from 'pngjs'

const sourceURL = process.env.SOURCE_BASE_URL
const rustURL = process.env.PLAYWRIGHT_BASE_URL
  ?? 'http://127.0.0.1:3200/arcademic-rust/'
const transparentImage = Buffer.from(
  '<svg xmlns="http://www.w3.org/2000/svg" width="1" height="1" viewBox="0 0 1 1"/>',
)

type Theme = 'light' | 'dark'
type VisualState = {
  name: string
  viewport: { width: number, height: number }
  theme: Theme
  prepare?: (page: Page, implementation: 'source' | 'rust') => Promise<void>
  stableClip?: 'activity' | 'chart' | 'publication'
}

async function createStablePage(
  browser: Browser,
  url: string,
  viewport: VisualState['viewport'],
  theme: Theme,
) {
  const context = await browser.newContext({ viewport, reducedMotion: 'reduce', colorScheme: theme })
  const page = await context.newPage()
  await page.route('https://images.unsplash.com/**', (route) => route.fulfill({
    body: transparentImage,
    contentType: 'image/svg+xml',
    status: 200,
  }))
  await page.goto(url, { waitUntil: 'networkidle' })
  await expect(page.getByRole('heading', { level: 1, name: 'Tony Stark' })).toBeVisible()
  const rustRoot = page.locator('.app-root')
  if (await rustRoot.count()) {
    await expect(rustRoot).toHaveAttribute('data-theme', theme)
    await expect.poll(() => page.locator('[data-reveal].reveal-pending').count())
      .toBeGreaterThan(0)
    await expect(page.locator('.activity-canvas')).toHaveClass(/is-ready/)
  }
  await page.addStyleTag({ content: `
    html { scroll-behavior: auto !important; }
    .blinking-grid, .pointer-glow, img { visibility: hidden !important; }
    *, *::before, *::after {
      animation-delay: 0s !important;
      animation-duration: 0s !important;
      caret-color: transparent !important;
      transition-delay: 0s !important;
      transition-duration: 0s !important;
    }
  ` })
  return { context, page }
}

async function activateAllSections(page: Page) {
  for (const name of [
    'Latest News',
    'Selected Research',
    'Teaching at Stark Industries',
    'GitHub Activity',
  ]) {
    const heading = page.getByRole('heading', { name })
    await heading.scrollIntoViewIfNeeded()
    await expect(heading).toBeVisible()
  }
  await page.locator('[style*="opacity"]').evaluateAll((nodes) => {
    for (const node of nodes) {
      const element = node as HTMLElement
      if (element.classList.contains('opacity-60')) continue
      element.style.opacity = '1'
      element.style.transform = 'none'
    }
  })
  await page.locator('.reveal-pending').evaluateAll((nodes) => {
    for (const node of nodes) {
      node.classList.remove('reveal-pending')
      node.classList.add('is-visible')
    }
  })
  const canvas = page.locator('#activity canvas')
  if (await canvas.count()) {
    await expect.poll(() => canvas.evaluate((node: HTMLCanvasElement) => {
      const context = node.getContext('2d')
      if (!context) return 0
      const ratio = node.width / 777
      return context.getImageData(
        Math.floor(771 * ratio),
        Math.floor(96 * ratio),
        1,
        1,
      ).data[3]
    })).toBe(128)
  }
}

async function centerInViewport(locator: Locator) {
  await locator.evaluate((node) => {
    const bounds = node.getBoundingClientRect()
    const target = bounds.top + window.scrollY - (window.innerHeight - bounds.height) / 2
    window.scrollTo(0, target)
  })
}

async function capture(
  browser: Browser,
  state: VisualState,
  implementation: 'source' | 'rust',
  url: string,
) {
  const { context, page } = await createStablePage(browser, url, state.viewport, state.theme)
  try {
    await activateAllSections(page)
    await page.evaluate(() => window.scrollTo(0, 0))
    await expect.poll(() => page.evaluate(() => window.scrollY)).toBe(0)
    if (state.prepare) await state.prepare(page, implementation)
    if (state.stableClip === 'publication') {
      return await page.getByRole('heading', {
        name: 'Particle acceleration in self-sustaining fusion reactions',
        exact: true,
      }).locator('../..').screenshot({ animations: 'disabled' })
    }
    if (state.stableClip === 'activity') {
      const canvas = page.locator('#activity canvas')
      let centerX: number
      let cellTop: number
      if (await canvas.count()) {
        const bounds = await canvas.boundingBox()
        expect(bounds).not.toBeNull()
        centerX = bounds!.x + 156
        cellTop = bounds!.y + 30
      } else {
        const bounds = await page.getByRole('gridcell').nth(72).boundingBox()
        expect(bounds).not.toBeNull()
        centerX = bounds!.x + bounds!.width / 2
        cellTop = bounds!.y
      }
      return await page.screenshot({
        animations: 'disabled',
        clip: { x: centerX - 75, y: cellTop - 40, width: 150, height: 80 },
      })
    }
    if (state.stableClip === 'chart') {
      const chart = page.getByRole('heading', { name: 'Research Output' }).locator('../..')
      const bounds = await chart.boundingBox()
      expect(bounds).not.toBeNull()
      const top = Math.max(0, bounds!.y - 48)
      return await page.screenshot({
        animations: 'disabled',
        clip: {
          x: bounds!.x,
          y: top,
          width: bounds!.width,
          height: bounds!.height + bounds!.y - top,
        },
      })
    }
    return await page.screenshot({ animations: 'disabled' })
  } finally {
    await context.close()
  }
}

async function compareBuffers(
  source: Buffer,
  rust: Buffer,
  name: string,
  testInfo: TestInfo,
) {
  const sourcePng = PNG.sync.read(source)
  const rustPng = PNG.sync.read(rust)
  expect(rustPng.width, `${name} width`).toBe(sourcePng.width)
  expect(rustPng.height, `${name} height`).toBe(sourcePng.height)

  const diff = new PNG({ width: sourcePng.width, height: sourcePng.height })
  const differentPixels = pixelmatch(
    sourcePng.data,
    rustPng.data,
    diff.data,
    sourcePng.width,
    sourcePng.height,
    { includeAA: false, threshold: 0.2 },
  )
  const ratio = differentPixels / (sourcePng.width * sourcePng.height)
  await testInfo.attach(`${name}-source`, { body: source, contentType: 'image/png' })
  await testInfo.attach(`${name}-rust`, { body: rust, contentType: 'image/png' })
  writeFileSync(testInfo.outputPath(`${name}-source.png`), source)
  writeFileSync(testInfo.outputPath(`${name}-rust.png`), rust)
  writeFileSync(testInfo.outputPath(`${name}-diff.png`), PNG.sync.write(diff))
  await testInfo.attach(`${name}-diff`, {
    body: PNG.sync.write(diff),
    contentType: 'image/png',
  })
  expect(ratio, `${name} pixel difference ratio`).toBeLessThanOrEqual(0.005)
}

const desktop = { width: 1440, height: 1000 }
const mobile = { width: 390, height: 844 }

const states: VisualState[] = [
  { name: 'desktop-light', viewport: desktop, theme: 'light' },
  { name: 'desktop-dark', viewport: desktop, theme: 'dark' },
  { name: 'mobile-light', viewport: mobile, theme: 'light' },
  { name: 'mobile-dark', viewport: mobile, theme: 'dark' },
  {
    name: 'mobile-menu',
    viewport: mobile,
    theme: 'light',
    prepare: async (page) => {
      await page.getByRole('button', { name: 'Open navigation' }).click()
      await expect(page.getByRole('button', { name: 'Close navigation' })).toBeVisible()
    },
  },
  {
    name: 'cv-hover',
    viewport: desktop,
    theme: 'light',
    prepare: async (page) => {
      await page.getByRole('link', { name: 'CV', exact: true }).hover()
    },
  },
  {
    name: 'publication-hover',
    viewport: desktop,
    theme: 'light',
    stableClip: 'publication',
    prepare: async (page) => {
      const card = page.getByRole('heading', {
        name: 'Particle acceleration in self-sustaining fusion reactions',
        exact: true,
      }).locator('../..')
      await centerInViewport(card)
      const bounds = await card.boundingBox()
      expect(bounds).not.toBeNull()
      await page.mouse.move(bounds!.x + bounds!.width / 2, bounds!.y + bounds!.height / 2)
      await expect(page.getByRole('heading', {
        name: 'Particle acceleration in self-sustaining fusion reactions',
      })).toBeVisible()
    },
  },
  {
    name: 'chart-tooltip',
    viewport: desktop,
    theme: 'light',
    stableClip: 'chart',
    prepare: async (page) => {
      const chart = page.getByRole('heading', { name: 'Research Output' }).locator('../..')
      await centerInViewport(chart)
      const year = page.getByText('2024', { exact: true }).last()
      await year.hover()
      await expect(page.getByText('18 Papers', { exact: true })).toBeVisible()
    },
  },
  {
    name: 'heatmap-hover',
    viewport: desktop,
    theme: 'light',
    stableClip: 'activity',
    prepare: async (page, implementation) => {
      await centerInViewport(page.locator('#activity .glass-card'))
      if (implementation === 'source') {
        const canvas = page.getByRole('img', { name: /52 weeks of contribution activity/ })
        const box = await canvas.boundingBox()
        expect(box).not.toBeNull()
        await canvas.dispatchEvent('pointermove', {
          bubbles: true,
          clientX: box!.x + 156,
          clientY: box!.y + 36,
          pointerType: 'mouse',
        })
        await expect(page.locator('.heatmap-tooltip')).toHaveClass(/opacity-100/)
      } else {
        const canvas = page.locator('.activity-canvas')
        const box = await canvas.boundingBox()
        expect(box).not.toBeNull()
        await canvas.dispatchEvent('pointermove', {
          bubbles: true,
          clientX: box!.x + 156,
          clientY: box!.y + 36,
          pointerType: 'mouse',
        })
        await expect(page.locator('.activity-canvas-tooltip')).toHaveClass(/is-visible/)
      }
    },
  },
  {
    name: 'scroll-to-top',
    viewport: desktop,
    theme: 'light',
    prepare: async (page) => {
      await page.evaluate(() => window.scrollTo(0, 301))
      await expect(page.getByTitle('Scroll to top')).toBeVisible()
    },
  },
]

for (const state of states) {
  test(`${state.name} stays within the source pixel budget`, async ({ browser }, testInfo) => {
    test.skip(!sourceURL, 'SOURCE_BASE_URL is required for source-to-Rust visual comparison')
    const source = await capture(browser, state, 'source', sourceURL!)
    const rust = await capture(browser, state, 'rust', rustURL)
    await compareBuffers(source, rust, state.name, testInfo)
  })
}

test('desktop and mobile layout landmarks match the source', async ({ browser }) => {
  test.skip(!sourceURL, 'SOURCE_BASE_URL is required for source-to-Rust visual comparison')

  for (const viewport of [desktop, mobile]) {
    const source = await createStablePage(browser, sourceURL!, viewport, 'light')
    const rust = await createStablePage(browser, rustURL, viewport, 'light')
    try {
      await activateAllSections(source.page)
      await activateAllSections(rust.page)
      const selectors = ['nav', '#home', '#news', '#publications', '#teaching', '#activity']
      for (const selector of selectors) {
        const sourceBox = await source.page.locator(selector).evaluate((node) => {
          const bounds = node.getBoundingClientRect()
          return {
            height: bounds.height,
            y: bounds.top + (getComputedStyle(node).position === 'fixed' ? 0 : window.scrollY),
          }
        })
        const rustBox = await rust.page.locator(selector).evaluate((node) => {
          const bounds = node.getBoundingClientRect()
          return {
            height: bounds.height,
            y: bounds.top + (getComputedStyle(node).position === 'fixed' ? 0 : window.scrollY),
          }
        })
        expect(sourceBox, `${selector} source bounds`).not.toBeNull()
        expect(rustBox, `${selector} Rust bounds`).not.toBeNull()
        expect.soft(Math.abs(sourceBox!.y - rustBox!.y), `${selector} top at ${viewport.width}px`)
          .toBeLessThanOrEqual(2)
        expect.soft(Math.abs(sourceBox!.height - rustBox!.height), `${selector} height at ${viewport.width}px`)
          .toBeLessThanOrEqual(2)
      }
    } finally {
      await source.context.close()
      await rust.context.close()
    }
  }
})
