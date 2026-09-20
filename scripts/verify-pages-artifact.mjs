#!/usr/bin/env node

import { readFileSync, readdirSync, statSync } from 'node:fs'
import { basename, join, relative, resolve, sep } from 'node:path'

const [directoryArgument, basePathArgument = '', cnameArgument] = process.argv.slice(2)
if (!directoryArgument) {
  throw new Error('usage: verify-pages-artifact.mjs <directory> [base-path]')
}

const root = resolve(directoryArgument)
const normalizedBase = basePathArgument.split('/').filter(Boolean).join('/')
const prefix = normalizedBase ? `/${normalizedBase}/` : '/'

function filesBelow(directory) {
  return readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const path = join(directory, entry.name)
    return entry.isDirectory() ? filesBelow(path) : [path]
  })
}

const files = filesBelow(root)
const relativeFiles = files.map((file) => relative(root, file)).sort()
const expectedRoot = ['.nojekyll', '404.html', 'index.html']
if (cnameArgument) expectedRoot.push('CNAME')
for (const required of expectedRoot) {
  if (!relativeFiles.includes(required)) throw new Error(`missing ${required}`)
}

const assets = relativeFiles.filter((file) => file.startsWith(`assets${sep}`))
const extensions = assets.map((file) => file.slice(file.lastIndexOf('.'))).sort()
if (assets.length !== 8 || extensions.join(',') !== '.css,.jpg,.js,.png,.png,.svg,.svg,.wasm') {
  throw new Error(`unexpected asset inventory: ${assets.join(', ')}`)
}
const expectedFileCount = cnameArgument ? 12 : 11
if (relativeFiles.length !== expectedFileCount) {
  throw new Error(`unexpected artifact inventory: ${relativeFiles.join(', ')}`)
}
if (cnameArgument && readFileSync(join(root, 'CNAME'), 'utf8').trim() !== cnameArgument) {
  throw new Error('CNAME does not match the configured custom domain')
}
for (const asset of assets) {
  if (!/-dxh[0-9a-f]+\.[a-z0-9]+$/i.test(asset)) {
    throw new Error(`asset is missing a content hash: ${asset}`)
  }
}
if (files.some((file) => (statSync(file).mode & 0o111) !== 0)) {
  throw new Error('Pages artifact contains an executable file')
}

const budgets = new Map([
  ['.wasm', 900_000],
  ['.js', 70_000],
  ['.css', 40_000],
  ['.jpg', 60_000],
])
for (const asset of assets) {
  const extension = asset.slice(asset.lastIndexOf('.'))
  const budget = budgets.get(extension)
  const size = statSync(join(root, asset)).size
  if (budget && size > budget) {
    throw new Error(`${asset} exceeds its ${budget}-byte budget: ${size}`)
  }
}

const index = readFileSync(join(root, 'index.html'), 'utf8')
const notFound = readFileSync(join(root, '404.html'), 'utf8')
if (Buffer.byteLength(index) > 40_000) {
  throw new Error(`index.html exceeds its 40000-byte budget: ${Buffer.byteLength(index)}`)
}
for (const token of [
  '<title>Xuning TAN</title>',
  '<meta name="google-site-verification" content="j-zSpLhdAjkV6kQEJ9w032KVBetc5hpdwjg5XUz4zkU"/>',
  'Xuning TAN',
  'Experience &#38; Education',
  'The Australian National University',
  'https://github.com/A1pine',
  'GitHub Contributions',
  'Loading GitHub contribution data...',
]) {
  if (!index.includes(token)) throw new Error(`index is missing SSR token: ${token}`)
}
const faviconTag = index.match(/<link\b[^>]*\brel="icon"[^>]*>/)?.[0] ?? ''
if (
  !faviconTag.includes('type="image/png"')
  || !faviconTag.includes('sizes="256x256"')
  || !/href="[^"]+favicon-[^"]+\.png"/.test(faviconTag)
) {
  throw new Error('index is missing the PNG favicon metadata')
}
for (const token of ['Tony Stark', 'id="publications"', 'id="teaching"']) {
  if (index.includes(token)) throw new Error(`index contains disabled or stale content: ${token}`)
}
if (!/<link rel="preload" href="[^"]+\.wasm" as="fetch" type="application\/wasm" crossorigin>/.test(index)) {
  throw new Error('index is missing the WASM preload')
}
if (!notFound.includes(`0; url=${prefix}`) || !notFound.includes('data-pages-404="true"')) {
  throw new Error(`404 does not redirect to ${prefix}`)
}

const localReferences = [...index.matchAll(/(?:href|src)="(\/[^"?#]+)/g)]
  .map((match) => match[1])
  .filter((reference) => reference.includes('/assets/'))
if (!localReferences.length) throw new Error('index has no local asset references')
for (const reference of localReferences) {
  if (!reference.startsWith(`${prefix}assets/`)) {
    throw new Error(`asset omits configured base path: ${reference}`)
  }
  const relativeAsset = reference.slice(prefix.length)
  if (!relativeFiles.includes(relativeAsset)) {
    throw new Error(`referenced asset is missing: ${reference}`)
  }
}

console.log(`${basename(root)}: ${relativeFiles.length} files, ${assets.length} current assets`)
