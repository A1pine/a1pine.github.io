import assert from 'node:assert/strict'
import test from 'node:test'

import {
  applyPublicationOverrides,
  articleImageUrl,
  inferPublicationType,
  toPublication,
} from '../scripts/publication-data.mjs'

test('infers journal, conference, and preprint types from publication metadata', () => {
  assert.equal(inferPublicationType({}, 'Renewable Energy, 126139'), 'journal')
  assert.equal(inferPublicationType({}, 'International Conference on Power Systems'), 'conference')
  assert.equal(inferPublicationType({}, 'arXiv preprint arXiv:2609.16887'), 'preprint')
  assert.equal(inferPublicationType({}, ''), 'other')
})

test('uses only explicit image metadata and image resources', () => {
  assert.equal(articleImageUrl({ thumbnail: 'https://example.org/cover.jpg' }), 'https://example.org/cover.jpg')
  assert.equal(articleImageUrl({ resources: [{ file_format: 'PDF', link: 'https://example.org/paper.pdf' }] }), '')
  assert.equal(articleImageUrl({ resources: [{ file_format: 'PNG', link: 'https://example.org/figure.png' }] }), 'https://example.org/figure.png')
})

test('manual overrides survive synchronization and derive code availability', () => {
  const publication = toPublication({
    citation_id: 'author:paper',
    title: 'A Paper',
    publication: 'Journal of Tests, 2026',
    year: 2026,
    authors: 'A Author, B Writer',
    link: 'https://scholar.google.com/paper',
  }, 0)
  const [overridden] = applyPublicationOverrides([publication], {
    'author:paper': {
      image_url: 'https://example.org/cover.jpg',
      image_alt: 'Paper cover',
      code_url: 'https://github.com/example/paper',
      publication_type: 'conference',
      tags: ['Energy'],
    },
  })

  assert.equal(overridden.image_url, 'https://example.org/cover.jpg')
  assert.equal(overridden.code_url, 'https://github.com/example/paper')
  assert.equal(overridden.code_available, true)
  assert.equal(overridden.publication_type, 'conference')
  assert.deepEqual(overridden.tags, ['Energy'])
})
