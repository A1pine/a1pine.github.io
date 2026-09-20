import { expect, test, type Page } from '@playwright/test'

import { activateSection, baseURL, openSite, siteUrl, watchRuntime } from './helpers'

const transparentImage = Buffer.from(
  '<svg xmlns="http://www.w3.org/2000/svg" width="1" height="1" viewBox="0 0 1 1"/>',
)
const vibeBadge = Buffer.from(
  '<svg xmlns="http://www.w3.org/2000/svg" width="320" height="22"><rect width="320" height="22" fill="#18181b"/></svg>',
)
const contributionStart = Date.UTC(2025, 7, 24)
const contributionFixture = Array.from({ length: 371 }, (_, index) => ({
  date: new Date(contributionStart + index * 86_400_000).toISOString().slice(0, 10),
  count: index === 370 ? 3 : index % 29 === 0 ? 1 : 0,
  level: index === 370 ? 2 : index % 29 === 0 ? 1 : 0,
}))

test.beforeEach(async ({ page }) => {
  await page.route('https://images.unsplash.com/**', (route) => route.fulfill({
    body: transparentImage,
    contentType: 'image/svg+xml',
    status: 200,
  }))
  await page.route('https://github.com/A1pine.png**', (route) => route.fulfill({
    body: transparentImage,
    contentType: 'image/svg+xml',
    status: 200,
  }))
  await page.route('https://vibecafe.ai/@a1pine/badge', (route) => route.fulfill({
    body: vibeBadge,
    contentType: 'image/svg+xml',
    status: 200,
  }))
  await page.route('https://github-contributions-api.jogruber.de/**', (route) => route.fulfill({
    body: JSON.stringify({ total: { lastYear: 210 }, contributions: contributionFixture }),
    contentType: 'application/json',
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

test('renders the configured personal profile with stable media geometry', async ({ page }) => {
  const assertRuntime = watchRuntime(page)
  await openSite(page)

  await activateSection(page, 'Experience & Education')
  await activateSection(page, 'Selected Research')
  await activateSection(page, 'GitHub Contributions')

  await expect(page.locator('#experience article')).toHaveCount(4)
  const advisorLinks = page.locator('.news-description-link')
  await expect(advisorLinks).toHaveCount(2)
  await expect(advisorLinks.nth(0)).toHaveText('Prof. Junhua Zhao')
  await expect(advisorLinks.nth(0)).toHaveAttribute('href', 'https://www.zhaojunhua.org/')
  await expect(advisorLinks.nth(1)).toHaveText('Prof. Jianwei Huang')
  await expect(advisorLinks.nth(1)).toHaveAttribute('href', 'https://jianwei.cuhk.edu.cn/')
  await expect(page.getByRole('heading', { name: 'Selected Research' })).toBeVisible()
  await expect(page.locator('.publication-empty')).toBeVisible()
  await expect(page.locator('.publications-updated')).toHaveCount(0)
  await expect(page.locator('#teaching')).toHaveCount(0)
  await expect(page.locator('.activity-summary')).toHaveAttribute('data-live-state', 'loaded')
  await expect(page.locator('.activity-summary')).toHaveText('210 contributions in the last year')
  const vibeLink = page.getByRole('link', { name: 'VibeUsage for the last 7 days' })
  await expect(vibeLink).toHaveAttribute('href', 'https://vibecafe.ai/@a1pine?ref=badge')
  await expect(page.locator('.vibe-usage-link + .activity-heading-row')).toHaveCount(1)
  await expect(page.getByRole('img', { name: 'VibeUsage for the last 7 days' })).toBeVisible()
  await expect(page.getByRole('gridcell')).toHaveCount(371)
  await expect(page.locator('.heatmap-cell[data-date="2026-08-29"]'))
    .toHaveAttribute('title', '2026-08-29: 3 contributions')

  const heroImage = page.getByRole('img', { name: 'Xuning TAN' })
  const heroBox = await heroImage.boundingBox()
  expect(heroBox).not.toBeNull()
  expect(Math.abs(heroBox!.width - heroBox!.height)).toBeLessThanOrEqual(1)
  await expect.poll(() => heroImage.evaluate((image) => getComputedStyle(image).filter))
    .toBe('saturate(0.68) contrast(0.97) brightness(1)')
  await heroImage.hover()
  await expect.poll(() => heroImage.evaluate((image) => getComputedStyle(image).filter))
    .toBe('saturate(1) contrast(1) brightness(1)')

  const biographyEmphasis = page.locator('.bio-card em')
  await expect(biographyEmphasis).toHaveCount(3)
  await expect(biographyEmphasis.nth(0)).toHaveText(
    'The Chinese University of Hong Kong, Shenzhen (CUHK-Shenzhen)',
  )
  await expect(biographyEmphasis.nth(1)).toHaveText('Shenzhen Loop Area Institute (SLAI)')
  await expect(biographyEmphasis.nth(2)).toHaveText('NetEase Games')
  for (const institution of await biographyEmphasis.all()) {
    await expect.poll(() => institution.evaluate((node) => getComputedStyle(node).fontStyle))
      .toBe('italic')
  }

  await expect(page.getByRole('link', { name: 'GitHub link: A1pine' }))
    .toHaveAttribute('href', 'https://github.com/A1pine')
  await expect(page.getByRole('link', { name: 'Twitter link: @XuningTan' }))
    .toHaveAttribute('href', 'https://x.com/XuningTan')
  await expect(page.locator('.social-list > .social-link').nth(1))
    .toHaveAttribute('href', 'https://x.com/XuningTan')
  await expect(page.locator('link[rel="icon"]')).toHaveAttribute('type', 'image/png')
  await expect(page.locator('link[rel="icon"]')).toHaveAttribute('sizes', '256x256')
  await expect(page.locator('link[rel="icon"]')).toHaveAttribute('href', /favicon-.+\.png$/)
  const googleVerification = page.locator('meta[name="google-site-verification"]')
  await expect(googleVerification).toHaveCount(1)
  await expect(googleVerification).toHaveAttribute(
    'content',
    'j-zSpLhdAjkV6kQEJ9w032KVBetc5hpdwjg5XUz4zkU',
  )

  const footer = page.getByRole('contentinfo')
  await expect(footer).toContainText(
    '© Copyright 2026 Xuning TAN. Powered by Rust, Vue and Dioxus. Hosted with Love.',
  )
  await expect(footer.locator('.footer-brand-icon')).toHaveCount(3)
  for (const icon of await footer.locator('.footer-brand-icon').all()) {
    expect(await icon.evaluate((image: HTMLImageElement) => image.naturalWidth)).toBeGreaterThan(0)
  }
  await expect(footer.locator('.footer-heart svg')).toHaveCount(1)
  expect(await page.evaluate(() => document.documentElement.scrollWidth))
    .toBe(await page.evaluate(() => document.documentElement.clientWidth))
  expect(await textOverflowingContainers(page)).toEqual([])
  await assertRuntime()
})

test('keeps motion and transition effects under the performance profile', async ({ page }) => {
  await page.emulateMedia({ reducedMotion: 'no-preference' })
  await openSite(page)

  await expect.poll(() => page.locator('.entrance-scale').evaluate((node) => ({
    name: getComputedStyle(node).animationName,
    duration: getComputedStyle(node).animationDuration,
  }))).toEqual({ name: 'entrance-scale', duration: '0.8s' })
  await expect.poll(() => page.locator('.portrait').evaluate((node) =>
    getComputedStyle(node).transitionDuration)).toContain('0.5s')
  await expect.poll(() => page.locator('.blinking-cell').count()).toBeGreaterThan(0)

  await activateSection(page, 'GitHub Contributions')
  await expect(page.locator('.activity-summary')).toHaveAttribute('data-live-state', 'loaded')
  await expect.poll(() => page.locator('.heatmap-cell').first().evaluate((node) =>
    getComputedStyle(node).animationName)).toBe('heatmap-enter')

  await page.emulateMedia({ colorScheme: 'dark' })
  await expect(page.locator('.app-root')).toHaveAttribute('data-theme', 'dark')
  await expect.poll(() => page.locator('.background-color').evaluate((node) =>
    getComputedStyle(node).transitionDuration)).toContain('1s')
})

test('keeps a stable contribution grid when the live API is unavailable', async ({ page }) => {
  await page.unroute('https://github-contributions-api.jogruber.de/**')
  await page.route('https://github-contributions-api.jogruber.de/**', (route) => route.fulfill({
    body: '{"malformed":true}',
    contentType: 'application/json',
    status: 200,
  }))
  const assertRuntime = watchRuntime(page)
  await openSite(page)

  await expect(page.locator('.activity-summary')).toHaveAttribute('data-live-state', 'error')
  await expect(page.locator('.activity-summary'))
    .toHaveText('Contribution data is temporarily unavailable.')
  await expect(page.getByRole('gridcell')).toHaveCount(0)
  await expect(page.locator('.heatmap-skeleton')).toHaveCount(1)
  await assertRuntime()
})

test('follows the system theme without client-side persistence', async ({ page }) => {
  const assertRuntime = watchRuntime(page)
  await page.emulateMedia({ colorScheme: 'light' })
  await openSite(page)

  const root = page.locator('.app-root')
  await expect(root).toHaveAttribute('data-theme', 'light')
  await expect(page.getByRole('button', { name: 'Toggle Theme' })).toHaveCount(0)
  expect(await page.evaluate(() => ({
    cookies: document.cookie,
    local: localStorage.length,
    session: sessionStorage.length,
  }))).toEqual({ cookies: '', local: 0, session: 0 })

  await page.emulateMedia({ colorScheme: 'dark' })
  await expect(root).toHaveAttribute('data-theme', 'dark')
  await expect.poll(() => page.evaluate(() => getComputedStyle(document.body).backgroundColor))
    .toBe('rgb(5, 9, 20)')

  await page.emulateMedia({ colorScheme: 'light' })
  await expect(root).toHaveAttribute('data-theme', 'light')
  await expect.poll(() => page.evaluate(() => getComputedStyle(document.body).backgroundColor))
    .toBe('rgb(248, 250, 252)')

  const glow = page.locator('#pointer-glow')
  const initialTransform = await glow.evaluate((node) => (node as HTMLElement).style.transform)
  await page.mouse.move(120, 140)
  await expect.poll(() => glow.evaluate((node) => (node as HTMLElement).style.transform))
    .not.toBe(initialTransform)
  await expect.poll(() => page.locator('.blinking-cell').count()).toBeGreaterThan(0)
  await expect(page.locator('.status-dot')).toHaveAttribute('title', /.+/)

  await activateSection(page, 'Experience & Education')
  const experienceCard = page.getByRole('article').first().locator('.news-card')
  await experienceCard.hover()
  await expect.poll(() => experienceCard.evaluate((node) => getComputedStyle(node).transform))
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
  expect(await page.evaluate(() => ({
    cookies: document.cookie,
    local: localStorage.length,
    session: sessionStorage.length,
  }))).toEqual({ cookies: '', local: 0, session: 0 })
  await assertRuntime()
})

test('follows browser language changes without client-side persistence', async ({ page }) => {
  await page.addInitScript(() => {
    const state = { languages: ['zh-CN', 'en-US'] }
    Object.defineProperty(window, '__testLanguages', { configurable: true, value: state })
    Object.defineProperty(navigator, 'languages', {
      configurable: true,
      get: () => window.__testLanguages.languages,
    })
    Object.defineProperty(navigator, 'language', {
      configurable: true,
      get: () => window.__testLanguages.languages[0],
    })
  })
  const assertRuntime = watchRuntime(page)
  await page.goto(siteUrl, { waitUntil: 'networkidle' })

  await expect(page.locator('html')).toHaveAttribute('lang', 'zh-CN')
  await expect(page.locator('.app-root')).toHaveAttribute('data-locale', 'zh-CN')
  await expect(page).toHaveTitle('谈旭宁')
  await expect(page.getByRole('heading', { level: 1, name: '谈旭宁' })).toBeVisible()
  await expect(page.getByRole('link', { name: '首页', exact: true })).toBeVisible()
  await expect(page.getByText('港中深 & SLAI', { exact: true })).toBeVisible()

  const biography = page.locator('.bio-card p')
  await expect(biography).toContainText('香港中文大学（深圳）')
  await expect(biography).toContainText('深圳河套学院（SLAI）')
  await expect(biography).toContainText('网易游戏')
  await expect(biography.locator('em')).toHaveText([
    '香港中文大学（深圳）',
    '深圳河套学院（SLAI）',
    '网易游戏',
  ])

  await expect(page.getByRole('heading', { name: '经历与教育' })).toBeVisible()
  await expect(page.getByRole('heading', { name: '网易游戏，广州' })).toBeVisible()
  await expect(page.getByText('任职于网易互娱，大话事业部', { exact: true })).toBeVisible()
  await expect(page.getByRole('heading', { name: '香港中文大学（深圳）' })).toBeVisible()
  await expect(page.getByRole('heading', {
    name: '港中深 & 深圳河套学院（SLAI）',
  })).toBeVisible()

  await expect(page.locator('.activity-summary')).toHaveAttribute('data-live-state', 'loaded')
  await expect(page.locator('.activity-summary')).toHaveText('过去一年共 210 次贡献')
  await expect(page.locator('.heatmap-cell[data-date="2026-08-29"]'))
    .toHaveAttribute('title', '2026-08-29：3 次贡献')
  await expect(page.getByRole('contentinfo')).toContainText(
    '© Copyright 2026 谈旭宁。由 Rust、Vue 和 Dioxus 驱动。用爱托管。',
  )
  await expect(page.getByTitle('返回顶部')).toHaveCount(1)
  expect(await page.evaluate(() => ({
    cookies: document.cookie,
    local: localStorage.length,
    session: sessionStorage.length,
  }))).toEqual({ cookies: '', local: 0, session: 0 })

  await page.setViewportSize({ width: 390, height: 844 })
  expect(await page.evaluate(() => document.documentElement.scrollWidth))
    .toBe(await page.evaluate(() => document.documentElement.clientWidth))
  expect(await textOverflowingContainers(page)).toEqual([])

  await page.evaluate(() => {
    window.__testLanguages.languages = ['en-US']
    window.dispatchEvent(new Event('languagechange'))
  })
  await expect(page.locator('html')).toHaveAttribute('lang', 'en')
  await expect(page.locator('.app-root')).toHaveAttribute('data-locale', 'en')
  await expect(page).toHaveTitle('Xuning TAN')
  await expect(page.getByRole('heading', { level: 1, name: 'Xuning TAN' })).toBeVisible()
  expect(await page.evaluate(() => ({
    cookies: document.cookie,
    local: localStorage.length,
    session: sessionStorage.length,
  }))).toEqual({ cookies: '', local: 0, session: 0 })
  await assertRuntime()
})

test('switches languages manually with a transition and remembers the choice', async ({ page }) => {
  const assertRuntime = watchRuntime(page)
  await openSite(page)
  await expect(page.locator('html')).toHaveAttribute('lang', 'en')

  await page.getByRole('button', { name: '中文' }).click()
  await expect(page.locator('html')).toHaveAttribute('lang', 'zh-CN')
  await expect(page.locator('.app-root')).toHaveAttribute('data-locale', 'zh-CN')
  await expect(page.locator('h1')).toHaveText('谈旭宁')
  await expect(page.getByRole('heading', { name: '经历与教育' })).toBeVisible()
  await expect(page.locator('.app-root')).not.toHaveClass(/locale-switching/)
  await expect.poll(() => page.evaluate(() => localStorage.getItem('arcademic-locale'))).toBe('zh')

  await page.reload({ waitUntil: 'networkidle' })
  await expect(page.locator('html')).toHaveAttribute('lang', 'zh-CN')
  await expect(page.locator('h1')).toHaveText('谈旭宁')

  await page.getByRole('button', { name: 'English' }).click()
  await expect(page.locator('html')).toHaveAttribute('lang', 'en')
  await expect(page.locator('h1')).toHaveText('Xuning TAN')
  await expect(page.locator('.app-root')).not.toHaveClass(/locale-switching/)
  await expect.poll(() => page.evaluate(() => localStorage.getItem('arcademic-locale'))).toBe('en')
  await assertRuntime()
})

test('supports keyboard-only mobile navigation', async ({
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
  await expect(page.getByRole('link', { name: 'Experience', exact: true })).toBeVisible()
  await expect(page.getByRole('link', { name: 'Activity', exact: true })).toBeVisible()
  await page.keyboard.press('Escape')
  await expect(menu).toBeFocused()
  await expect(menu).toHaveAttribute('aria-expanded', 'false')

  const activity = page.getByLabel('GitHub Contributions')
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

  await expect(page.getByRole('heading', { level: 1, name: 'Xuning TAN' })).toHaveCount(1)
  await expect(page.getByRole('heading', { name: 'Experience & Education' })).toHaveCount(1)
  await expect(page.getByRole('heading', { name: 'GitHub Contributions' })).toHaveCount(1)
  await expect(page.getByRole('heading', {
    level: 3,
    name: 'The Australian National University',
  })).toHaveCount(1)
  await expect(page.getByRole('heading', {
    level: 3,
    name: 'NetEase Games, Guangzhou',
  })).toHaveCount(1)
  await expect(page.locator('#publications')).toHaveCount(1)
  await expect(page.locator('#teaching')).toHaveCount(0)
  await expect(page.getByRole('gridcell')).toHaveCount(0)
  await expect(page.locator('.heatmap-skeleton')).toHaveCount(1)
  await expect(page.locator('.activity-summary')).toHaveText('Loading GitHub contribution data...')
  await expect(page.getByRole('contentinfo')).toContainText('Xuning TAN')

  const externalLinks = await page.locator('a[href^="https://"]').evaluateAll((nodes) =>
    nodes.map((node) => (node as HTMLAnchorElement).href),
  )
  expect(externalLinks.length).toBeGreaterThan(0)
  await context.close()
})

declare global {
  interface Window {
    __testLanguages: { languages: string[] }
  }
}
