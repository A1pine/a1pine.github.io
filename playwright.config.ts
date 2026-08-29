import { defineConfig, devices } from '@playwright/test'

const baseURL = process.env.PLAYWRIGHT_BASE_URL ?? 'http://127.0.0.1:3200/arcademic-rust/'

export default defineConfig({
  testDir: './tests/e2e',
  fullyParallel: true,
  forbidOnly: Boolean(process.env.CI),
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 2 : undefined,
  reporter: process.env.CI
    ? [['line'], ['html', { open: 'never' }]]
    : 'line',
  outputDir: 'test-results',
  use: {
    baseURL,
    reducedMotion: 'reduce',
    screenshot: 'only-on-failure',
    trace: 'retain-on-failure',
  },
  projects: [
    {
      name: 'chromium',
      testIgnore: /visual\.spec\.ts/,
      use: { ...devices['Desktop Chrome'] },
    },
    {
      name: 'firefox',
      testIgnore: /visual\.spec\.ts/,
      use: { ...devices['Desktop Firefox'] },
    },
    {
      name: 'webkit',
      testIgnore: /visual\.spec\.ts/,
      use: { ...devices['Desktop Safari'] },
    },
    {
      name: 'visual-chromium',
      testMatch: /visual\.spec\.ts/,
      use: { ...devices['Desktop Chrome'] },
    },
  ],
})
