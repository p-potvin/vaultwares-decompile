// Deconstructed — Local AI Model Client
// Connects to Ollama or a llama.cpp OpenAI-compatible server running locally.
// NO external API calls are made — privacy is guaranteed.

const OLLAMA_BASE = 'http://localhost:11434';
const LLAMACPP_BASE = 'http://localhost:8080';

/**
 * Detects which local AI backend is available.
 *
 * @returns {Promise<{ available: boolean, backend: 'ollama'|'llamacpp'|null, models: string[] }>}
 */
export async function detectBackend() {
  // Try Ollama first
  try {
    const res = await fetch(`${OLLAMA_BASE}/api/tags`, { signal: AbortSignal.timeout(3000) });
    if (res.ok) {
      const data = await res.json();
      const models = (data.models || []).map((m) => m.name);
      return { available: true, backend: 'ollama', models };
    }
  } catch {
    // Ollama not available — try llama.cpp
  }

  // Try llama.cpp (OpenAI-compatible)
  try {
    const res = await fetch(`${LLAMACPP_BASE}/v1/models`, { signal: AbortSignal.timeout(3000) });
    if (res.ok) {
      const data = await res.json();
      const models = (data.data || []).map((m) => m.id);
      return { available: true, backend: 'llamacpp', models };
    }
  } catch {
    // llama.cpp not available either
  }

  return { available: false, backend: null, models: [] };
}

/**
 * Generates a completion from the local AI model.
 *
 * @param {string} prompt - Complete prompt string (system + user merged)
 * @param {{ model: string, backend?: 'ollama'|'llamacpp', onToken?: (token: string) => void }} options
 * @returns {Promise<string>} - Complete response text
 */
export async function generate(prompt, options = {}) {
  const { model, onToken } = options;

  // Auto-detect backend if not specified
  const backend = options.backend ?? (await detectBackend()).backend;

  if (!backend) {
    throw new Error(
      'No local AI backend detected. Please install Ollama (https://ollama.com) or llama.cpp and ensure it is running.'
    );
  }

  if (backend === 'ollama') {
    return generateOllama(prompt, model, onToken);
  } else {
    return generateLlamaCpp(prompt, model, onToken);
  }
}

/**
 * Calls the Ollama `/api/generate` endpoint (streaming).
 *
 * @param {string} prompt
 * @param {string} model
 * @param {((token: string) => void) | undefined} onToken
 * @returns {Promise<string>}
 */
async function generateOllama(prompt, model, onToken) {
  const res = await fetch(`${OLLAMA_BASE}/api/generate`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ model, prompt, stream: Boolean(onToken) }),
  });

  if (!res.ok) throw new Error(`Ollama error ${res.status}: ${await res.text()}`);

  if (onToken) {
    // Streaming: read NDJSON lines
    const reader = res.body.getReader();
    const decoder = new TextDecoder();
    let full = '';
    while (true) {
      const { done, value } = await reader.read();
      if (done) break;
      const lines = decoder.decode(value).split('\n').filter(Boolean);
      for (const line of lines) {
        try {
          const obj = JSON.parse(line);
          if (obj.response) {
            onToken(obj.response);
            full += obj.response;
          }
        } catch { /* skip malformed lines */ }
      }
    }
    return full;
  } else {
    const data = await res.json();
    return data.response ?? '';
  }
}

/**
 * Calls the llama.cpp `/v1/completions` endpoint (OpenAI-compatible).
 *
 * @param {string} prompt
 * @param {string} model
 * @param {((token: string) => void) | undefined} onToken
 * @returns {Promise<string>}
 */
async function generateLlamaCpp(prompt, model, onToken) {
  const res = await fetch(`${LLAMACPP_BASE}/v1/completions`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      model: model || 'default',
      prompt,
      max_tokens: 2048,
      stream: false,
    }),
  });

  if (!res.ok) throw new Error(`llama.cpp error ${res.status}: ${await res.text()}`);
  const data = await res.json();
  return data.choices?.[0]?.text ?? '';
}
