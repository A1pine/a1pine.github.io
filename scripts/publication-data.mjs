const PUBLICATION_TYPES = new Set(['journal', 'conference', 'preprint', 'other'])
const OVERRIDABLE_FIELDS = [
  'publication_type',
  'description',
  'tags',
  'image_url',
  'image_alt',
  'pdf_url',
  'code_url',
]

export function parseInteger(value) {
  const match = String(value ?? '').replace(/\u00a0/g, ' ').match(/\d[\d,]*/)
  if (!match) return 0
  return Number.parseInt(match[0].replace(/,/g, ''), 10)
}

export function normalizeAuthor(author) {
  const trimmed = String(author).trim().replace(/\s+/g, ' ')
  return trimmed.replace(/^([A-Z])\s+([A-Z][\p{L}'-]*)$/u, '$1. $2')
}

export function splitAuthors(value) {
  return String(value ?? '')
    .split(/,\s*|\s+and\s+/i)
    .map(normalizeAuthor)
    .filter((author) => author.length > 0 && author !== '...')
}

export function slugify(title, index) {
  const slug = String(title)
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '')
    .slice(0, 64)
    .replace(/-+$/g, '')
  return `${slug || 'publication'}-${index + 1}`
}

export function parseVenue(venueLine) {
  const trimmed = String(venueLine ?? '').trim().replace(/\s+/g, ' ')
  const yearMatch = trimmed.match(/(?:^|,\s*|\s)(\d{4})(?:\s*$|,)/)
  if (!yearMatch) return { name: trimmed }
  const name = trimmed.slice(0, yearMatch.index).replace(/[,\s]+$/, '')
  return { name: name || trimmed }
}

export function inferPublicationType(article, venue) {
  const value = `${article.publication_type ?? article.type ?? ''} ${venue}`.toLowerCase()
  if (/\b(arxiv|preprint|working paper|ssrn)\b/.test(value)) return 'preprint'
  if (/\b(conference|proceedings|workshop|symposium|congress)\b/.test(value)) {
    return 'conference'
  }
  if (/\b(journal|transactions|letters|review)\b/.test(value) || venue) return 'journal'
  return 'other'
}

export function articleImageUrl(article) {
  for (const candidate of [article.image_url, article.image, article.thumbnail]) {
    if (typeof candidate === 'string' && /^https?:\/\//.test(candidate)) return candidate
  }
  const resources = Array.isArray(article.resources) ? article.resources : []
  const image = resources.find((resource) => (
    /^(?:image\/)?(?:png|jpe?g|webp)$/i.test(resource.file_format ?? resource.type ?? '')
    && /^https?:\/\//.test(resource.link ?? '')
  ))
  return image?.link ?? ''
}

export function toPublication(article, index) {
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

  const id = slugify(title, index)
  const sourceId = String(article.citation_id ?? article.result_id ?? id).trim() || id
  const imageUrl = articleImageUrl(article)
  return {
    id,
    source_id: sourceId,
    title,
    venue: venue || 'Google Scholar',
    publication_type: inferPublicationType(article, venue),
    year,
    authors,
    description: '',
    tags: [],
    image_url: imageUrl,
    image_alt: imageUrl ? `${title} publication image` : '',
    pdf_url: article.link ?? '',
    code_url: '',
    code_available: false,
    citations,
  }
}

export function applyPublicationOverrides(publications, overrides) {
  return publications.map((publication) => {
    const override = overrides[publication.source_id] ?? overrides[publication.id] ?? {}
    const merged = { ...publication }
    for (const field of OVERRIDABLE_FIELDS) {
      if (Object.hasOwn(override, field)) merged[field] = override[field]
    }
    if (!PUBLICATION_TYPES.has(merged.publication_type)) {
      throw new Error(
        `invalid publication_type for ${publication.source_id}: ${merged.publication_type}`,
      )
    }
    if (!Array.isArray(merged.tags) || merged.tags.some((tag) => typeof tag !== 'string')) {
      throw new Error(`tags override for ${publication.source_id} must be an array of strings`)
    }
    for (const field of ['image_url', 'image_alt', 'pdf_url', 'code_url', 'description']) {
      if (typeof merged[field] !== 'string') {
        throw new Error(`${field} override for ${publication.source_id} must be a string`)
      }
    }
    merged.code_available = merged.code_url.trim().length > 0
    if (merged.image_url && !merged.image_alt) merged.image_alt = `${merged.title} publication image`
    return merged
  })
}
