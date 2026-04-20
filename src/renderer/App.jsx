// Deconstructed — Root App Component
// VaultWares Cyberpunk Cinder theme applied globally.

import React, { useState } from 'react';
import CrawlerView from './views/CrawlerView.jsx';
import ApiExplorerView from './views/ApiExplorerView.jsx';
import JsDeobfuscatorView from './views/JsDeobfuscatorView.jsx';

// ── VaultWare Cyberpunk Cinder theme tokens ───────────────────────────────
const THEME = {
  background:     '#073642',
  surface:        '#0a3f4f',
  surfaceElevated:'#0d4a5a',
  textPrimary:    '#F8FAFC',
  textSecondary:  '#CBD5E1',
  accent:         '#CB4B16',
  accentHover:    '#e05a22',
  borderSubtle:   '#164f60',
  focusRing:      '#d96030',
  success:        '#16A34A',
  warning:        '#D97706',
  danger:         '#DC2626',
};

const VIEWS = ['crawler', 'api-explorer', 'js-deobfuscator'];
const VIEW_LABELS = {
  'crawler': '🕷 Crawler',
  'api-explorer': '📋 API Explorer',
  'js-deobfuscator': '🔍 JS Deobfuscator',
};

const styles = {
  app: {
    display: 'flex',
    flexDirection: 'column',
    height: '100vh',
    backgroundColor: THEME.background,
    color: THEME.textPrimary,
    fontFamily: "'Segoe UI', system-ui, sans-serif",
    fontSize: '14px',
  },
  header: {
    display: 'flex',
    alignItems: 'center',
    gap: '16px',
    padding: '10px 20px',
    backgroundColor: THEME.surfaceElevated,
    borderBottom: `1px solid ${THEME.borderSubtle}`,
    userSelect: 'none',
  },
  logo: {
    fontWeight: 700,
    fontSize: '16px',
    color: THEME.accent,
    letterSpacing: '0.04em',
  },
  nav: {
    display: 'flex',
    gap: '4px',
    marginLeft: '16px',
  },
  navButton: (active) => ({
    padding: '6px 14px',
    borderRadius: '4px',
    border: 'none',
    cursor: 'pointer',
    fontSize: '13px',
    fontWeight: active ? 600 : 400,
    backgroundColor: active ? THEME.accent : 'transparent',
    color: active ? '#fff' : THEME.textSecondary,
    transition: 'background-color 0.15s',
  }),
  content: {
    flex: 1,
    overflow: 'hidden',
    display: 'flex',
    flexDirection: 'column',
  },
  badge: {
    marginLeft: 'auto',
    fontSize: '11px',
    color: THEME.textSecondary,
    opacity: 0.7,
  },
};

export default function App() {
  const [activeView, setActiveView] = useState('crawler');
  const [crawlResult, setCrawlResult] = useState(null);

  return (
    <div style={styles.app}>
      <header style={styles.header}>
        <span style={styles.logo}>⬡ DECONSTRUCTED</span>
        <nav style={styles.nav}>
          {VIEWS.map((view) => (
            <button
              key={view}
              style={styles.navButton(activeView === view)}
              onClick={() => setActiveView(view)}
            >
              {VIEW_LABELS[view]}
            </button>
          ))}
        </nav>
        <span style={styles.badge}>VaultWares · Privacy First</span>
      </header>

      <main style={styles.content}>
        {activeView === 'crawler' && (
          <CrawlerView
            theme={THEME}
            onCrawlComplete={(result) => {
              setCrawlResult(result);
              setActiveView('api-explorer');
            }}
          />
        )}
        {activeView === 'api-explorer' && (
          <ApiExplorerView
            theme={THEME}
            routes={crawlResult?.routes || []}
          />
        )}
        {activeView === 'js-deobfuscator' && (
          <JsDeobfuscatorView
            theme={THEME}
            assets={crawlResult?.jsAssets || []}
          />
        )}
      </main>
    </div>
  );
}

export { THEME };
