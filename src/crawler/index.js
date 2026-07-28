/**
 * @typedef {Object} CrawlOptions
 * @property {number} maxPages        - Maximum pages to visit (default: 100)
 * @property {number} maxConcurrency  - Parallel browser tabs (default: 3)
 * @property {string[]} excludePatterns - URL patterns to skip (regex strings)
 * @property {boolean} followExternalLinks - Follow links outside the start domain
 */

/**
 * @typedef {Object} CrawlResult
 * @property {Object[]} routes   - All discovered API routes
 * @property {Object[]} jsAssets - All downloaded JS bundle assets
 * @property {string} sessionId  - UUID for this crawl session
 * @property {number} pagesVisited
 * @property {number} durationMs
 * @property {string} targetUrl
 */

/**
 * @typedef {Object} CrawlProgressEvent
 * @property {'route_found'|'asset_downloaded'|'page_crawled'|'error'|'complete'} type
 * @property {*} payload
 */

export { crawlSite } from './crawler.js';
