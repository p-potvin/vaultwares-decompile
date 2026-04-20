// Deconstructed — Network Request Interceptor
// Captures XHR and fetch calls made by crawled pages.

const SENSITIVE_HEADER_PATTERN = /^(authorization|cookie|x-api-key|x-auth-token|x-session)/i;

/**
 * Captures a Playwright request as a structured API call object.
 * Authorization and session header values are redacted for safety.
 *
 * @param {import('playwright').Request} playwrightRequest
 * @returns {RawRequest | null}
 */
export function interceptRequest(playwrightRequest) {
  try {
    const method = playwrightRequest.method().toUpperCase();
    const url = playwrightRequest.url();
    const rawHeaders = playwrightRequest.headers();
    const postData = playwrightRequest.postData();

    // Sanitise headers — redact sensitive values but preserve keys
    const headers = {};
    for (const [key, value] of Object.entries(rawHeaders)) {
      headers[key] = SENSITIVE_HEADER_PATTERN.test(key) ? '[REDACTED]' : value;
    }

    let body = null;
    if (postData) {
      try {
        body = JSON.parse(postData);
      } catch {
        body = postData; // keep as raw string if not JSON
      }
    }

    return { method, url, headers, body, capturedAt: new Date().toISOString() };
  } catch {
    return null;
  }
}

/**
 * @typedef {Object} RawRequest
 * @property {string} method
 * @property {string} url
 * @property {Record<string, string>} headers
 * @property {object|string|null} body
 * @property {string} capturedAt
 */
