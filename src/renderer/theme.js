// Deconstructed — Centralized Theme Module
// Derives all UI tokens from the vault-themes Brand assets.
// Default theme is Solarized Warm Light; dark themes remain available.
//
// Brand sources (vault-themes/Brand/):
//   codex-solarized-light-revisited.json  → Solarized Light palette
//   tokens.ts                             → VaultWares brand colours
//   tailwind.config.ts                    → Extended palette
//   brand.i18n.ts                         → Tagline and copy strings

// ── Brand colour primitives (from vault-themes/Brand/tokens.ts) ─────────────
const BRAND = {
  slate:    '#4A5459',
  gold:     '#CC9B21',
  goldMuted:'#B78C1E',
  goldLight:'#E5C06A',
  paper:    '#FDFCF7',
  ink:      '#002B36',
  cyan:     '#21B8CC',
  green:    '#4ECC21',
};

// ── Solarized palette (from vault-themes/Brand/codex-solarized-light-revisited.json) ──
const SOLARIZED = {
  base03:  '#002B36',
  base02:  '#073642',
  base01:  '#586E75',
  base00:  '#657B83',
  base0:   '#839496',
  base1:   '#93A1A1',
  base2:   '#EEE8D5',
  base3:   '#FDF6E3',
  yellow:  '#B58900',
  orange:  '#CB4B16',
  red:     '#DC322F',
  magenta: '#D33682',
  violet:  '#6C71C4',
  blue:    '#268BD2',
  cyan:    '#2AA198',
  green:   '#859900',
};

// ── Helper: blend two hex colours ───────────────────────────────────────────
function hexToRgb(hex) {
  const h = hex.replace('#', '');
  return [parseInt(h.slice(0, 2), 16), parseInt(h.slice(2, 4), 16), parseInt(h.slice(4, 6), 16)];
}

function blendHex(base, mix, ratio) {
  const [br, bg, bb] = hexToRgb(base);
  const [mr, mg, mb] = hexToRgb(mix);
  const r = Math.round(br + (mr - br) * ratio);
  const g = Math.round(bg + (mg - bg) * ratio);
  const b = Math.round(bb + (mb - bb) * ratio);
  return `#${r.toString(16).padStart(2, '0')}${g.toString(16).padStart(2, '0')}${b.toString(16).padStart(2, '0')}`;
}

// ── Theme definitions ───────────────────────────────────────────────────────

/**
 * Solarized Warm Light — the new default theme for Deconstructed.
 *
 * Warm, paper-toned background inspired by the Solarized Light palette with
 * VaultWares gold accent.  Optimised for extended reading sessions during
 * security research.
 */
export const SOLARIZED_WARM_LIGHT = {
  name:            'Solarized Warm Light',
  slug:            'solarized-warm-light',
  mode:            'light',
  background:      SOLARIZED.base3,            // #FDF6E3
  surface:         SOLARIZED.base2,            // #EEE8D5
  surfaceElevated: blendHex(SOLARIZED.base2, BRAND.gold, 0.06),
  textPrimary:     SOLARIZED.base03,           // #002B36
  textSecondary:   SOLARIZED.base00,           // #657B83
  accent:          BRAND.gold,                 // #CC9B21
  accentHover:     BRAND.goldMuted,            // #B78C1E
  borderSubtle:    blendHex(SOLARIZED.base2, SOLARIZED.base01, 0.18),
  focusRing:       blendHex(BRAND.gold, SOLARIZED.base03, 0.15),
  success:         '#15803D',
  warning:         '#B45309',
  danger:          '#B91C1C',
};

/**
 * Cyberpunk Cinder — the original dark theme.
 *
 * Deep blue-green background with orange accent.  Preserved for users who
 * prefer a dark UI.
 */
export const CYBERPUNK_CINDER = {
  name:            'Cyberpunk Cinder',
  slug:            'cyberpunk-cinder',
  mode:            'dark',
  background:      '#073642',
  surface:         '#0a3f4f',
  surfaceElevated: '#0d4a5a',
  textPrimary:     '#F8FAFC',
  textSecondary:   '#CBD5E1',
  accent:          '#CB4B16',
  accentHover:     '#e05a22',
  borderSubtle:    '#164f60',
  focusRing:       '#d96030',
  success:         '#16A34A',
  warning:         '#D97706',
  danger:          '#DC2626',
};

/**
 * Solarized Dark — a dark companion theme using the Solarized Dark palette.
 */
export const SOLARIZED_DARK = {
  name:            'Solarized Dark',
  slug:            'solarized-dark',
  mode:            'dark',
  background:      SOLARIZED.base03,           // #002B36
  surface:         SOLARIZED.base02,           // #073642
  surfaceElevated: blendHex(SOLARIZED.base02, SOLARIZED.base0, 0.10),
  textPrimary:     SOLARIZED.base2,            // #EEE8D5
  textSecondary:   SOLARIZED.base1,            // #93A1A1
  accent:          SOLARIZED.yellow,           // #B58900
  accentHover:     blendHex(SOLARIZED.yellow, '#FFFFFF', 0.14),
  borderSubtle:    blendHex(SOLARIZED.base02, SOLARIZED.base0, 0.18),
  focusRing:       blendHex(SOLARIZED.yellow, '#FFFFFF', 0.22),
  success:         SOLARIZED.green,            // #859900
  warning:         SOLARIZED.orange,           // #CB4B16
  danger:          SOLARIZED.red,              // #DC322F
};

// ── Theme registry ──────────────────────────────────────────────────────────

const THEMES = [
  SOLARIZED_WARM_LIGHT,
  SOLARIZED_DARK,
  CYBERPUNK_CINDER,
];

/** Default theme slug. */
export const DEFAULT_THEME_SLUG = 'solarized-warm-light';

/**
 * Returns the theme that matches the given slug, or the default theme.
 * @param {string} slug
 * @returns {object} theme token map
 */
export function getTheme(slug) {
  const normalised = (slug || '').trim().toLowerCase();
  return THEMES.find((t) => t.slug === normalised) || SOLARIZED_WARM_LIGHT;
}

/**
 * Returns a shallow copy of all available themes.
 * @returns {object[]}
 */
export function listThemes() {
  return [...THEMES];
}

/**
 * VaultWares brand font stack (from vault-themes/Brand/tokens.ts).
 */
export const FONT_FAMILY = "'Segoe UI Semilight', 'Inter', system-ui, sans-serif";

/**
 * Monospace font stack for code views.
 */
export const FONT_FAMILY_MONO = "ui-monospace, 'JetBrains Mono', 'Cascadia Code', Menlo, monospace";
