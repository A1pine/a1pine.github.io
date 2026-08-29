#!/usr/bin/env node

import { readFileSync, readdirSync, statSync } from 'node:fs'
import { basename, join, relative, resolve, sep } from 'node:path'

const [directoryArgument, basePathArgument = ''] = process.argv.slice(2)
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
for (const required of expectedRoot) {
  if (!relativeFiles.includes(required)) throw new Error(`missing ${required}`)
}

const assets = relativeFiles.filter((file) => file.startsWith(`assets${sep}`))
const extensions = assets.map((file) => file.slice(file.lastIndexOf('.'))).sort()
if (assets.length !== 4 || extensions.join(',') !== '.css,.ico,.js,.wasm') {
  throw new Error(`unexpected asset inventory: ${assets.join(', ')}`)
}
if (relativeFiles.length !== 7) {
  throw new Error(`unexpected artifact inventory: ${relativeFiles.join(', ')}`)
}
if (files.some((file) => (statSync(file).mode & 0o111) !== 0)) {
  throw new Error('Pages artifact contains an executable file')
}

const index = readFileSync(join(root, 'index.html'), 'utf8')
const notFound = readFileSync(join(root, '404.html'), 'utf8')
for (const token of ['Tony Stark', 'Latest News', 'Selected Research', 'GitHub Activity']) {
  if (!index.includes(token)) throw new Error(`index is missing SSR token: ${token}`)
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
