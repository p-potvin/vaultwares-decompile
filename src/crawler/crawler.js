// Deconstructed — Crawlee-based Site Spider
// Discovers routes and downloads JS assets from a target website.

import { PlaywrightCrawler, RequestQueue } from 'crawlee';
import { v4 as uuidv4 } from 'uuid';
import { interceptRequest } from './api-interceptor.js';
import { addRoute } from './route-mapper.js';
import { downloadJsAsset } from './js-downloader.js';

/**
 * Crawls a website and returns all discovered API routes and JS assets.
 *
 * @param {string} startUrl
 * @param {CrawlOptions} options
 * @param {(event: CrawlProgressEvent) => void} onProgress
 * @returns {Promise<CrawlResult>}
 */
export async function crawlSite(startUrl, options = {}, onProgress = () => {}) {
  const {
    maxPages = 100,
    maxConcurrency = 3,
    excludePatterns = [],
    followExternalLinks = false,
  } = options;

  const sessionId = uuidv4();
  const startTime = Date.now();
  const routes = [];
  const jsAssets = [];
  let pagesVisited = 0;

  const startDomain = new URL(startUrl).hostname;

  const requestQueue = await RequestQueue.open();
  await requestQueue.addRequest({ url: startUrl });

  const crawler = new PlaywrightCrawler({
    requestQueue,
    maxRequestsPerCrawl: maxPages,
    maxConcurrency,

    async requestHandler({ page, request, enqueueLinks }) {
      pagesVisited++;

      onProgress({ type: 'page_crawled', payload: { url: request.url, count: pagesVisited } });

      // ── Intercept all network requests on this page ───────────────────────
      await page.route('**/*', async (route) => {
        const req = route.request();
        const resourceType = req.resourceType();

        // Only capture XHR / fetch calls as API routes
        if (resourceType === 'xhr' || resourceType === 'fetch') {
          const captured = interceptRequest(req);
          if (captured) {
            const route = addRoute(captured, routes);
            if (route.isNew) {
              onProgress({ type: 'route_found', payload: route.schema });
            }
          }
        }

        // Download JS assets
        if (resourceType === 'script') {
          const url = req.url();
          if (!url.startsWith('data:')) {
            const asset = await downloadJsAsset(url, sessionId);
            if (asset) {
              jsAssets.push(asset);
              onProgress({ type: 'asset_downloaded', payload: asset });
            }
          }
        }

        await route.continue();
      });

      // ── Discover new links ────────────────────────────────────────────────
      await enqueueLinks({
        strategy: followExternalLinks ? 'all' : 'same-domain',
        exclude: excludePatterns.map((p) => new RegExp(p)),
      });
    },

    async failedRequestHandler({ request }) {
      onProgress({ type: 'error', payload: { url: request.url, errorMessage: request.errorMessages?.[0] } });
    },
  });

  await crawler.run();

  const result = {
    sessionId,
    routes,
    jsAssets,
    pagesVisited,
    durationMs: Date.now() - startTime,
    targetUrl: startUrl,
  };

  onProgress({ type: 'complete', payload: { sessionId, routeCount: routes.length, assetCount: jsAssets.length } });

  return result;
}

/**
 * @typedef {Object} CrawlOptions
 * @property {number} maxPages
 * @property {number} maxConcurrency
 * @property {string[]} excludePatterns
 * @property {boolean} followExternalLinks
 */

/**
 * @typedef {Object} CrawlResult
 * @property {string} sessionId
 * @property {import('./route-mapper.js').RouteSchema[]} routes
 * @property {import('./js-downloader.js').JsAsset[]} jsAssets
 * @property {number} pagesVisited
 * @property {number} durationMs
 * @property {string} targetUrl
 */

/**
 * @typedef {Object} CrawlProgressEvent
 * @property {'page_crawled'|'route_found'|'asset_downloaded'|'error'|'complete'} type
 * @property {*} payload
 */
