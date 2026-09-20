import { expect, type Page } from '@playwright/test'

export const baseURL = process.env.PLAYWRIGHT_BASE_URL
  ?? 'http://127.0.0.1:3200/arcademic-rust/'
export const siteUrl = './'

export function watchRuntime(page: Page) {
  const consoleErrors: string[] = []
  const pageErrors: string[] = []
  const failedRequests: string[] = []

  page.on('console', (message) => {
    if (message.type() === 'error') consoleErrors.push(message.text())
  })
  page.on('pageerror', (error) => pageErrors.push(error.message))
  page.on('response', (response) => {
    if (response.status() >= 400 && response.url().startsWith('http://127.0.0.1')) {
      failedRequests.push(`${response.status()} ${response.url()}`)
    }
  })

  return async () => {
    await expect.poll(() => consoleErrors, { message: 'browser console errors' }).toEqual([])
    expect(pageErrors, 'uncaught page errors').toEqual([])
    expect(failedRequests, 'failed first-party requests').toEqual([])
  }
}

export async function openSite(page: Page) {
  await page.goto(siteUrl, { waitUntil: 'networkidle' })
  await expect(page).toHaveTitle('Xuning TAN')
  await expect(page.getByRole('heading', { level: 1, name: 'Xuning TAN' })).toBeVisible()
}

export async function activateSection(page: Page, heading: string) {
  const target = page.getByRole('heading', { name: heading })
  await target.scrollIntoViewIfNeeded()
  await expect(target).toBeVisible()
}
