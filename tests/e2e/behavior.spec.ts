import { expect, test, type Page } from '@playwright/test'

import { activateSection, baseURL, openSite, siteUrl, watchRuntime } from './helpers'

const transparentImage = Buffer.from(
  '<svg xmlns="http://www.w3.org/2000/svg" width="1" height="1" viewBox="0 0 1 1"/>',
)

test.beforeEach(async ({ page }) => {
  await page.route('https://images.unsplash.com/**', (route) => route.fulfill({
    body: transparentImage,
    contentType: 'image/svg+xml',
    status: 200,
  }))
})

async function textOverflowingContainers(page: Page) {
  return await page.evaluate(() => [...document.querySelectorAll<HTMLElement>(
    'h1, h2, h3, p, a, button, span, time',
  )]
    .filter((node) => !node.matches('.course-card h3, .course-description'))
    .filter((node) => {
      const style = getComputedStyle(node)
      return Boolean(node.textContent?.trim())
        && style.display !== 'none'
        && style.visibility !== 'hidden'
        && style.transform === 'none'
        && node.clientWidth > 0
        && (node.scrollWidth > node.clientWidth + 1 || node.scrollHeight > node.clientHeight + 1)
    })
    .map((node) => ({ className: node.className, text: node.textContent?.trim() })))
}

test('renders every configured module with stable media geometry', async ({ page }) => {
  const assertRuntime = watchRuntime(page)
  await openSite(page)

  for (const heading of [
    'Latest News',
    'Selected Research',
    'Teaching at Stark Industries',
    'GitHub Activity',
  ]) {
    await activateSection(page, heading)
  }

  await expect(page.getByRole('article')).toHaveCount(10)
  await expect(page.getByRole('gridcell')).toHaveCount(364)
  await expect(page.getByRole('button', { name: /PDF/ })).toHaveCount(3)
  await expect(page.getByRole('button', { name: /Code/ })).toHaveCount(3)
  const publicationActions = page.getByRole('button', { name: /PDF|Code/ })
  for (let index = 0; index < await publicationActions.count(); index += 1) {
    await expect(publicationActions.nth(index)).toBeDisabled()
  }

  const heroImage = page.getByRole('img', { name: 'Tony Stark' })
  const heroBox = await heroImage.boundingBox()
  expect(heroBox).not.toBeNull()
  expect(Math.abs(heroBox!.width - heroBox!.height)).toBeLessThanOrEqual(1)

  const publicationImage = page.getByRole('img', {
    name: 'Earth viewed from space',
  })
  const publicationBox = await publicationImage.boundingBox()
  expect(publicationBox).not.toBeNull()
  expect(publicationBox!.width).toBeGreaterThan(0)
  expect(publicationBox!.height).toBeGreaterThan(0)
  expect(await page.evaluate(() => document.documentElement.scrollWidth))
    .toBe(await page.evaluate(() => document.documentElement.clientWidth))
  expect(await textOverflowingContainers(page)).toEqual([])
  await assertRuntime()
})

test('operates theme, pointer, chart, heatmap, and scroll controls', async ({ page }) => {
  const assertRuntime = watchRuntime(page)
  await openSite(page)

  const root = page.locator('.app-root')
  const themeButton = page.getByRole('button', { name: 'Toggle Theme' }).first()
  const initialTheme = await root.getAttribute('data-theme')
  await themeButton.click()
  await expect(root).not.toHaveAttribute('data-theme', initialTheme ?? 'system')
  const selectedTheme = await root.getAttribute('data-theme')
  await page.reload({ waitUntil: 'networkidle' })
  await expect(root).toHaveAttribute('data-theme', selectedTheme!)

  const glow = page.locator('#pointer-glow')
  const initialTransform = await glow.evaluate((node) => (node as HTMLElement).style.transform)
  await page.mouse.move(120, 140)
  await expect.poll(() => glow.evaluate((node) => (node as HTMLElement).style.transform))
    .not.toBe(initialTransform)
  await expect.poll(() => page.locator('.blinking-cell').count()).toBeGreaterThan(0)
  await expect(page.locator('.status-dot')).toHaveAttribute('title', /.+/)

  await activateSection(page, 'Latest News')
  const newsCard = page.getByRole('article').first().locator('.news-card')
  await newsCard.hover()
  await expect.poll(() => newsCard.evaluate((node) => getComputedStyle(node).transform))
    .not.toBe('none')

  await activateSection(page, 'Selected Research')
  const newestYear = page.getByText('2024', { exact: true }).last()
  await newestYear.hover()
  await expect(page.getByText('18 Papers', { exact: true })).toBeVisible()

  await activateSection(page, 'GitHub Activity')
  const canvas = page.locator('.activity-canvas')
  await expect(canvas).toHaveClass(/is-ready/)
  const darkPixel = await canvas.evaluate((node: HTMLCanvasElement) => {
    const ratio = node.width / 777
    return [...(node.getContext('2d')?.getImageData(
      Math.floor(156 * ratio),
      Math.floor(36 * ratio),
      1,
      1,
    ).data ?? [])]
  })
  expect(darkPixel).toEqual([52, 211, 153, 255])
  await canvas.evaluate((node) => {
    const bounds = node.getBoundingClientRect()
    node.dispatchEvent(new PointerEvent('pointermove', {
      bubbles: true,
      clientX: bounds.left + 6,
      clientY: bounds.top + 6,
      pointerType: 'mouse',
    }))
  })
  await expect(page.locator('.activity-canvas-tooltip')).toHaveClass(/is-visible/)
  await expect(page.locator('.activity-canvas-tooltip')).toHaveText('Activity Level: 0')

  await activateSection(page, 'Teaching at Stark Industries')
  const course = page.getByRole('heading', { name: 'Introduction to Arc Reactor Technology' })
    .locator('..')
  await course.hover()
  await expect.poll(() => course.evaluate((node) => getComputedStyle(node).transform))
    .not.toBe('none')

  const scrollTop = page.getByTitle('Scroll to top')
  await page.evaluate(() => window.scrollTo(0, 300))
  await expect(scrollTop).toHaveAttribute('aria-hidden', 'true')
  await page.evaluate(() => window.scrollTo(0, 301))
  await expect(scrollTop).toHaveAttribute('aria-hidden', 'false')
  await expect.poll(() => page.locator('.reading-progress').evaluate(
    (node) => getComputedStyle(node).transform,
  )).not.toBe('matrix(0, 0, 0, 1, 0, 0)')
  await scrollTop.click()
  await expect.poll(() => page.evaluate(() => window.scrollY)).toBe(0)
  await assertRuntime()
})

test('supports keyboard-only mobile navigation and local horizontal scrolling', async ({
  browserName,
  page,
}) => {
  const assertRuntime = watchRuntime(page)
  await page.setViewportSize({ width: 390, height: 844 })
  await openSite(page)

  await page.keyboard.press(browserName === 'webkit' ? 'Alt+Tab' : 'Tab')
  await expect(page.getByText('Skip to main content', { exact: true })).toBeFocused()
  await page.keyboard.press('Enter')
  await expect(page.getByRole('main')).toBeFocused()

  const menu = page.getByRole('button', { name: 'Open navigation' })
  await menu.focus()
  await page.keyboard.press('Enter')
  await expect(page.getByRole('button', { name: 'Close navigation' })).toBeVisible()
  await expect(page.getByRole('link', { name: 'Research', exact: true })).toBeVisible()
  await page.keyboard.press('Escape')
  await expect(menu).toBeFocused()
  await expect(menu).toHaveAttribute('aria-expanded', 'false')

  const chart = page.getByLabel('Research Output')
  await chart.scrollIntoViewIfNeeded()
  await chart.focus()
  await page.keyboard.press('ArrowRight')
  expect(await chart.evaluate((node) => node.scrollWidth >= node.clientWidth)).toBe(true)

  const activity = page.getByLabel('GitHub Activity')
  await activity.scrollIntoViewIfNeeded()
  await activity.focus()
  const before = await activity.evaluate((node) => node.scrollLeft)
  await page.keyboard.press('ArrowRight')
  await expect.poll(() => activity.evaluate((node) => node.scrollLeft)).toBeGreaterThan(before)

  const widths = await page.evaluate(() => ({
    client: document.documentElement.clientWidth,
    scroll: document.documentElement.scrollWidth,
  }))
  expect(widths.scroll).toBe(widths.client)
  expect(await textOverflowingContainers(page)).toEqual([])
  await assertRuntime()
})

test('keeps complete semantic content without JavaScript', async ({ browser }) => {
  const context = await browser.newContext({ baseURL, javaScriptEnabled: false })
  const page = await context.newPage()
  await page.goto(siteUrl, { waitUntil: 'load' })

  await expect(page.getByRole('heading', { level: 1, name: 'Tony Stark' })).toHaveCount(1)
  await expect(page.getByRole('heading', { name: 'Latest News' })).toHaveCount(1)
  await expect(page.getByRole('heading', { name: 'Selected Research' })).toHaveCount(1)
  await expect(page.getByRole('heading', { name: 'Teaching at Stark Industries' })).toHaveCount(1)
  await expect(page.getByRole('heading', { name: 'GitHub Activity' })).toHaveCount(1)
  await expect(page.getByRole('gridcell')).toHaveCount(364)

  const externalLinks = await page.locator('a[href^="https://"]').evaluateAll((nodes) =>
    nodes.map((node) => (node as HTMLAnchorElement).href),
  )
  expect(externalLinks.length).toBeGreaterThan(0)
  await context.close()
})
