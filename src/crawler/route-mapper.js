// Deconstructed — Route Mapper
// Normalises captured API requests into a deduplicated route map.

import { v4 as uuidv4 } from 'uuid';

/** @type {RegExp} Matches positive integers (likely IDs) */
const ID_PATTERN = /^[0-9]+$/;
/** @type {RegExp} Matches UUID v1–v5 */
const UUID_PATTERN = /^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i;
/** @type {RegExp} Matches alphanumeric IDs (8-20 chars, MUST contain both letters AND digits) */
const ALPHANUMERIC_ID_PATTERN = /^(?=[a-zA-Z0-9]{8,20}$)(?=.*[a-zA-Z])(?=.*[0-9])[a-zA-Z0-9]+$/;
/** @type {RegExp} Matches kebab-case slugs — MUST contain at least one hyphen */
const SLUG_PATTERN = /^[a-z][a-z0-9]*(-[a-z][a-z0-9]*)+$/;

/**
 * Normalises a URL path, replacing dynamic segments with parameter placeholders.
 *
 * @param {string} urlString
 * @returns {{ pattern: string, pathParams: string[] }}
 */
export function normalisePath(urlString) {
  let pathname;
  try {
    pathname = new URL(urlString).pathname;
  } catch {
    pathname = urlString;
  }

  const segments = pathname.split('/').filter(Boolean);
  const pathParams = [];

  const normalisedSegments = segments.map((segment) => {
    if (ID_PATTERN.test(segment)) {
      pathParams.push(':id');
      return ':id';
    }
    if (UUID_PATTERN.test(segment)) {
      pathParams.push(':uuid');
      return ':uuid';
    }
    if (ALPHANUMERIC_ID_PATTERN.test(segment)) {
      pathParams.push(':id');
      return ':id';
    }
    if (SLUG_PATTERN.test(segment)) {
      pathParams.push(':slug');
      return ':slug';
    }
    return segment;
  });

  return {
    pattern: '/' + normalisedSegments.join('/'),
    pathParams: [...new Set(pathParams)],
  };
}

/**
 * Adds a captured raw request to the routes array, deduplicating by method+pattern.
 * Returns the route schema and whether it is a newly discovered route.
 *
 * @param {import('./api-interceptor.js').RawRequest} rawRequest
 * @param {RouteSchema[]} routes - Mutated in place
 * @returns {{ schema: RouteSchema, isNew: boolean }}
 */
export function addRoute(rawRequest, routes) {
  const { pattern, pathParams } = normalisePath(rawRequest.url);
  const key = `${rawRequest.method}:${pattern}`;

  const existing = routes.find((r) => r.key === key);
  if (existing) {
    existing.interceptedCount++;
    // Merge any new headers or body keys seen in later calls
    if (rawRequest.headers) {
      Object.assign(existing.headers, rawRequest.headers);
    }
    return { schema: existing, isNew: false };
  }

  /** @type {RouteSchema} */
  const schema = {
    id: uuidv4(),
    key,
    method: rawRequest.method,
    url: rawRequest.url,
    pattern,
    pathParams,
    headers: rawRequest.headers || {},
    body: rawRequest.body,
    interceptedCount: 1,
    firstSeenAt: rawRequest.capturedAt,
  };

  routes.push(schema);
  return { schema, isNew: true };
}

/**
 * Groups routes by their base path segment (first non-empty segment).
 *
 * @param {RouteSchema[]} routes
 * @returns {Record<string, RouteSchema[]>}
 */
export function groupRoutes(routes) {
  return routes.reduce((groups, route) => {
    const base = route.pattern.split('/')[1] || '/';
    if (!groups[base]) groups[base] = [];
    groups[base].push(route);
    return groups;
  }, {});
}

/**
 * @typedef {Object} RouteSchema
 * @property {string} id
 * @property {string} key            - Deduplication key (method:pattern)
 * @property {string} method
 * @property {string} url            - First observed raw URL
 * @property {string} pattern        - Normalised path pattern
 * @property {string[]} pathParams
 * @property {Record<string, string>} headers
 * @property {object|string|null} body
 * @property {number} interceptedCount
 * @property {string} firstSeenAt
 */
