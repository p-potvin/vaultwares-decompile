// Deconstructed — AI Rename Prompt Builder
// Constructs prompts that instruct a local LLM to produce a JSON identifier rename map.

const SYSTEM_PROMPT = `You are a JavaScript reverse-engineering assistant. Your only job is to analyse obfuscated JavaScript code and propose human-readable names for obfuscated identifiers.

RULES:
1. Respond with ONLY valid JSON — no explanation, no prose, no code blocks.
2. JSON schema: { "renames": [ { "original": "<name>", "proposed": "<name>", "confidence": 0.0-1.0 } ] }
3. Only rename identifiers that are single letters, short nonsense names, or clearly machine-generated.
4. Never rename: undefined, null, true, false, NaN, Infinity, arguments, this, super, eval, constructor, prototype.
5. Confidence: 0.9+ = very sure, 0.7–0.9 = likely, below 0.7 = uncertain (these will be flagged for review).
6. If you are unsure about a name, omit it rather than guess.
7. Infer meaning from how the identifier is used (e.g. assigned to element.style, called as a callback, etc.).

FEW-SHOT EXAMPLES:

Example 1:
Code:
  var a = document.getElementById('login');
  var b = a.value;
  if (b.length < 8) { c(b); }
Identifiers to rename: a, b, c
Output:
  {"renames":[{"original":"a","proposed":"loginInput","confidence":0.95},{"original":"b","proposed":"inputValue","confidence":0.9},{"original":"c","proposed":"handleShortInput","confidence":0.7}]}

Example 2:
Code:
  function e(t, r) {
    return t.map(function(n) { return n * r; });
  }
Identifiers to rename: e, t, r, n
Output:
  {"renames":[{"original":"e","proposed":"scaleArray","confidence":0.85},{"original":"t","proposed":"array","confidence":0.9},{"original":"r","proposed":"scaleFactor","confidence":0.88},{"original":"n","proposed":"item","confidence":0.9}]}

Example 3:
Code:
  var x = {};
  x.q = function(z) { fetch('/api/auth', { method: 'POST', body: JSON.stringify(z) }); };
Identifiers to rename: x, q, z
Output:
  {"renames":[{"original":"x","proposed":"authService","confidence":0.92},{"original":"q","proposed":"loginRequest","confidence":0.88},{"original":"z","proposed":"credentials","confidence":0.91}]}
`;

/**
 * Builds a complete prompt (system + user) for identifier renaming.
 *
 * @param {string} codeChunk     - A chunk of beautified but still obfuscated JS
 * @param {string[]} identifiers - List of identifiers to rename in this chunk
 * @returns {string}
 */
export function buildRenamePrompt(codeChunk, identifiers) {
  const identifierList = identifiers.join(', ');
  return (
    SYSTEM_PROMPT +
    '\n\n--- CODE TO ANALYSE ---\n' +
    codeChunk +
    '\n\n--- IDENTIFIERS TO RENAME ---\n' +
    identifierList +
    '\n\n--- OUTPUT (JSON only) ---\n'
  );
}

/**
 * Parses a model response string into an array of rename proposals.
 * Returns an empty array if the response is not valid JSON.
 *
 * @param {string} responseText
 * @returns {Array<{ original: string, proposed: string, confidence: number }>}
 */
export function parseRenameResponse(responseText) {
  // Strip any accidental markdown code fences
  const cleaned = responseText
    .replace(/^```(?:json)?\s*/i, '')
    .replace(/\s*```\s*$/, '')
    .trim();

  try {
    const parsed = JSON.parse(cleaned);
    if (!Array.isArray(parsed.renames)) return [];
    return parsed.renames.filter(
      (r) =>
        typeof r.original === 'string' &&
        typeof r.proposed === 'string' &&
        typeof r.confidence === 'number'
    );
  } catch {
    return [];
  }
}
