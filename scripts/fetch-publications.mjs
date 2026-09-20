#!/usr/bin/env node

import { chromium } from '@playwright/test'
import { mkdir, readFileSync, writeFile } from 'node:fs'
import { dirname, resolve } from 'node:path'

const scholarId = process.argv[2] ?? process.env.SCHOLAR_ID
const serpApiKey = process.env.SERPAPI_KEY
const outputPath = resolve(
  process.argv[3] ?? process.env.PUBLICATIONS_PATH ?? 'config/publications.json',
)

if (!scholarId) {
  throw new Error('usage: fetch-publications.mjs <scholar-id> [output]\n       set SCHOLAR_ID, and optionally SERPAPI_KEY')
}

const profileUrl = `https://scholar.google.com/citations?user=${encodeURIComponent(scholarId)}&hl=en&pagesize=100&sortby=pubdate&view_op=list_works`

function parseInteger(value) {
  const match = String(value ?? '').replace(/\u00a0/g, ' ').match(/\d[\d,]*/)
  if (!match) return 0
  return Number.parseInt(match[0].replace(/,/g, ''), 10)
}

function normalizeAuthor(author) {
  const trimmed = String(author).trim().replace(/\s+/g, ' ')
  return trimmed.replace(/^([A-Z])\s+([A-Z][\p{L}'-]*)$/u, '$1. $2')
}

function splitAuthors(value) {
  return String(value ?? '')
    .split(/,\s*|\s+and\s+/i)
    .map(normalizeAuthor)
    .filter((author) => author.length > 0 && author !== '...')
}

function slugify(title, index) {
  const slug = String(title)
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '')
    .slice(0, 64)
    .replace(/-+$/g, '')
  return `${slug || 'publication'}-${index + 1}`
}

function parseVenue(venueLine) {
  const trimmed = String(venueLine ?? '').trim().replace(/\s+/g, ' ')
  const yearMatch = trimmed.match(/(?:^|,\s*|\s)(\d{4})(?:\s*$|,)/)
  if (!yearMatch) return { name: trimmed }
  const name = trimmed.slice(0, yearMatch.index).replace(/[,\s]+$/, '')
  return { name: name || trimmed }
}

function toPublication(article, index) {
  const title = String(article.title ?? '').trim()
  const authors = Array.isArray(article.authors)
    ? article.authors.map((author) => normalizeAuthor(author.name)).filter(Boolean)
    : splitAuthors(article.authors)
  const venue = article.venue || parseVenue(article.publication ?? article.bib?.citation ?? '').name
  const year = parseInteger(article.year ?? article.bib?.pub_year)
  const citations = parseInteger(
    article.cited_by?.value ?? article.num_citations ?? 0,
  )
  if (!title || !year) return null

  return {
    id: slugify(title, index),
    title,
    venue: venue || 'Google Scholar',
    year,
    authors,
    description: '',
    tags: [],
    image_url: '',
    image_alt: '',
    pdf_url: article.link ?? '',
    code_url: '',
    citations,
  }
}

async function fetchWithSerpApi(apiKey) {
  const publications = []
  for (let start = 0; start < 500; start += 20) {
    const url = new URL('https://serpapi.com/search.json')
    url.searchParams.set('engine', 'google_scholar_author')
    url.searchParams.set('author_id', scholarId)
    url.searchParams.set('hl', 'en')
    url.searchParams.set('start', String(start))
    url.searchParams.set('api_key', apiKey)
    const response = await fetch(url, { headers: { accept: 'application/json' } })
    const body = await response.text()
    if (!response.ok) {
      throw new Error(`SerpAPI request failed (${response.status}): ${body.slice(0, 300)}`)
    }
    const data = JSON.parse(body)
    const articles = Array.isArray(data.articles) ? data.articles : []
    const page = articles
      .map(toPublication)
      .filter((item) => item !== null)
    publications.push(...page)
    if (articles.length < 20) break
  }
  if (!publications.length) {
    throw new Error(`SerpAPI returned no publications for ${scholarId}`)
  }
  console.log(`SerpAPI: fetched ${publications.length} publications`)
  return publications
}

async function fetchWithBrowser() {
  const browser = await chromium.launch({
    headless: true,
    args: ['--disable-blink-features=AutomationControlled'],
  })
  const context = await browser.newContext({
    viewport: { width: 1280, height: 900 },
    userAgent:
      'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36',
    locale: 'en-US',
  })
  await context.addCookies([{
    name: 'SOCS',
    value: 'CAESEwgDEgk2NzM3OTg2NzUaAmVuIAEaBgiA_LyaBg',
    domain: '.google.com',
    path: '/',
  }])
  const page = await context.newPage()
  page.setDefaultTimeout(45_000)

  async function loadProfile(attempt) {
    await page.goto(profileUrl, {
      waitUntil: 'domcontentloaded',
      timeout: 60_000,
    })
    if (page.url().includes('consent.google.com')) {
      const accept = page.getByRole('button', { name: /accept all|i agree|同意/i }).first()
      if (await accept.count()) {
        await accept.click()
        await page.waitForLoadState('domcontentloaded')
      } else {
        await page.locator('form[action*="consent"]').first().submit()
        await page.waitForLoadState('domcontentloaded')
      }
    }
    console.log(`attempt ${attempt}: ${page.url()} — ${await page.title()}`)
    await page
      .locator('#gsc_prf_in, .gsc_a_tr')
      .first()
      .waitFor({ state: 'visible', timeout: 30_000 })
  }

  try {
    await loadProfile(1)
  } catch (error) {
    console.warn(`first profile load failed: ${error.message}`)
    await page.waitForTimeout(10_000)
    try {
      await loadProfile(2)
    } catch (retryError) {
      const body = await page.locator('body').innerText().catch(() => '')
      console.error(`profile body: ${body.slice(0, 500)}`)
      throw new Error(
        'Google Scholar blocked the browser fetch. Set SERPAPI_KEY for reliable CI updates.',
        { cause: retryError },
      )
    }
  }

  let expanded = 0
  while (expanded < 20) {
    const moreButton = page.locator('#gsc_bpf_more')
    if (!(await moreButton.count()) || await moreButton.isDisabled()) break
    await moreButton.click()
    expanded += 1
    await page.waitForTimeout(500)
  }

  const rowCount = await page.locator('.gsc_a_tr').count()
  if (!rowCount) {
    throw new Error('Google Scholar profile has no publications')
  }

  const publications = []
  for (let index = 0; index < rowCount; index += 1) {
    const row = page.locator('.gsc_a_tr').nth(index)
    const titleAnchor = row.locator('.gsc_a_at')
    const title = (await titleAnchor.innerText()).trim()
    const href = await titleAnchor.getAttribute('href')
    const grayLines = await row.locator('.gs_gray').allInnerTexts()
    let authorsLine = grayLines[0] ?? ''
    const venueLine = grayLines.slice(1).join(', ')
    const year = parseInteger(await row.locator('.gsc_a_y').innerText())
    const citations = parseInteger(await row.locator('.gsc_a_c').innerText())

    if (/…|\.\.\./.test(authorsLine) && href) {
      const articleUrl = new URL(href, 'https://scholar.google.com/').toString()
      await page.goto(articleUrl, { waitUntil: 'domcontentloaded' })
      const fullAuthors = await page
        .locator('div.gsc_oci_field', { hasText: /^Authors$/ })
        .locator('xpath=following-sibling::div[1]')
        .innerText()
        .catch(() => '')
      if (fullAuthors) authorsLine = fullAuthors
      await page.goto(profileUrl, { waitUntil: 'domcontentloaded' })
      await page.waitForTimeout(300)
    }

    const publication = toPublication({
      title,
      link: href ? new URL(href, 'https://scholar.google.com/').toString() : '',
      authors: authorsLine,
      publication: venueLine,
      year,
      cited_by: { value: citations },
    }, index)
    if (publication) publications.push(publication)
  }

  await browser.close()
  console.log(`Browser: fetched ${publications.length} publications`)
  return publications
}

const publications = serpApiKey
  ? await fetchWithSerpApi(serpApiKey)
  : await fetchWithBrowser()

const sorted = publications.sort((left, right) => (
  right.year - left.year
  || right.citations - left.citations
  || left.title.localeCompare(right.title)
))
const currentYear = new Date().getUTCFullYear()
const recentYears = Array.from({ length: 5 }, (_, offset) => currentYear - offset)
const stats = recentYears
  .map((year) => ({ year, count: sorted.filter((item) => item.year === year).length }))
  .filter((stat) => stat.count > 0)

const output = {
  total_value: String(sorted.length),
  citations_value: String(sorted.reduce((total, item) => total + item.citations, 0)),
  last_updated: new Date().toISOString().slice(0, 10),
  stats,
  items: sorted,
}

mkdir(dirname(outputPath), { recursive: true })
let previous = null
try {
  previous = readFileSync(outputPath, 'utf8')
} catch {
  previous = null
}
const next = `${JSON.stringify(output, null, 2)}\n`
if (previous !== next) {
  writeFile(outputPath, next, 'utf8')
  console.log(`${outputPath}: updated ${sorted.length} publications`)
} else {
  console.log(`${outputPath}: already current (${sorted.length} publications)`)
}
