#!/usr/bin/env node

import { createReadStream, existsSync, realpathSync, statSync } from 'node:fs'
import { createServer } from 'node:http'
import { extname, join, resolve, sep } from 'node:path'

const [directoryArgument, portArgument = '3200', basePathArgument = ''] = process.argv.slice(2)
if (!directoryArgument) {
  throw new Error('usage: serve-pages.mjs <directory> [port] [base-path]')
}

const root = realpathSync(directoryArgument)
const port = Number.parseInt(portArgument, 10)
if (!Number.isInteger(port) || port < 1 || port > 65_535) {
  throw new Error(`invalid port: ${portArgument}`)
}

const normalizedBase = basePathArgument.split('/').filter(Boolean).join('/')
const prefix = normalizedBase ? `/${normalizedBase}` : ''
const contentTypes = new Map([
  ['.css', 'text/css; charset=utf-8'],
  ['.html', 'text/html; charset=utf-8'],
  ['.ico', 'image/x-icon'],
  ['.js', 'text/javascript; charset=utf-8'],
  ['.json', 'application/json; charset=utf-8'],
  ['.svg', 'image/svg+xml'],
  ['.wasm', 'application/wasm'],
])

function sendFile(request, response, file, status = 200) {
  const size = statSync(file).size
  response.writeHead(status, {
    'Content-Length': size,
    'Content-Type': contentTypes.get(extname(file)) ?? 'application/octet-stream',
  })
  if (request.method === 'HEAD') {
    response.end()
  } else {
    createReadStream(file).pipe(response)
  }
}

function sendNotFound(request, response) {
  const notFound = join(root, '404.html')
  if (existsSync(notFound)) {
    sendFile(request, response, notFound, 404)
  } else {
    response.writeHead(404, { 'Content-Type': 'text/plain; charset=utf-8' })
    response.end('Not Found')
  }
}

const server = createServer((request, response) => {
  if (request.method !== 'GET' && request.method !== 'HEAD') {
    response.writeHead(405, { Allow: 'GET, HEAD' })
    response.end()
    return
  }

  let pathname
  try {
    pathname = decodeURIComponent(new URL(request.url ?? '/', 'http://localhost').pathname)
  } catch {
    response.writeHead(400)
    response.end()
    return
  }

  if (prefix && pathname === prefix) {
    response.writeHead(301, { Location: `${prefix}/` })
    response.end()
    return
  }
  if (prefix && !pathname.startsWith(`${prefix}/`)) {
    sendNotFound(request, response)
    return
  }

  let relative = prefix ? pathname.slice(prefix.length) : pathname
  if (relative.endsWith('/')) relative += 'index.html'
  const candidate = resolve(root, `.${relative}`)
  if (candidate !== root && !candidate.startsWith(`${root}${sep}`)) {
    sendNotFound(request, response)
    return
  }

  if (existsSync(candidate) && statSync(candidate).isFile()) {
    sendFile(request, response, candidate)
  } else {
    sendNotFound(request, response)
  }
})

server.listen(port, '127.0.0.1', () => {
  console.log(`Pages preview: http://127.0.0.1:${port}${prefix}/`)
})

for (const signal of ['SIGINT', 'SIGTERM']) {
  process.on(signal, () => server.close(() => process.exit(0)))
}
