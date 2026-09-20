import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'

import { expect, test } from '@playwright/test'

import { baseURL, openSite, watchRuntime } from './helpers'

const axeSource = readFileSync(fileURLToPath(import.meta.resolve('axe-core/axe.min.js')), 'utf8')

test('has no axe violations in both themes and viewports', async ({ browser }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium')

  for (const [name, viewport, theme] of [
    ['desktop-light', { width: 1440, height: 1000 }, 'light'],
    ['desktop-dark', { width: 1440, height: 1000 }, 'dark'],
    ['mobile-light', { width: 390, height: 844 }, 'light'],
    ['mobile-dark', { width: 390, height: 844 }, 'dark'],
  ] as const) {
    const context = await browser.newContext({
      baseURL,
      viewport,
      reducedMotion: 'reduce',
      colorScheme: theme,
    })
    const page = await context.newPage()
    const assertRuntime = watchRuntime(page)
    await openSite(page)
    await page.addScriptTag({ content: axeSource })
    const violations = await page.evaluate(async () => {
      const result = await window.axe.run(document, { resultTypes: ['violations'] })
      return result.violations
    })
    expect(violations, name).toEqual([])
    await assertRuntime()
    await context.close()
  }
})

declare global {
  interface Window {
    axe: typeof import('axe-core')
  }
}
