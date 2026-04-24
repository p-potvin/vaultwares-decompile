// Deconstructed — Root App Component
// Theme tokens sourced from the centralized theme module.

import React, { useState } from 'react';
import { getTheme, listThemes, DEFAULT_THEME_SLUG, FONT_FAMILY } from './theme.js';
import CrawlerView from './views/CrawlerView.jsx';
import ApiExplorerView from './views/ApiExplorerView.jsx';
import JsDeobfuscatorView from './views/JsDeobfuscatorView.jsx';

const VIEWS = ['crawler', 'api-explorer', 'js-deobfuscator'];
const VIEW_LABELS = {
  'crawler': '🕷 Crawler',
  'api-explorer': '📋 API Explorer',
  'js-deobfuscator': '🔍 JS Deobfuscator',
};

function buildStyles(THEME) {
  return {
    app: {
      display: 'flex',
      flexDirection: 'column',
      height: '100vh',
      backgroundColor: THEME.background,
      color: THEME.textPrimary,
      fontFamily: FONT_FAMILY,
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
      color: active ? (THEME.mode === 'light' ? THEME.background : THEME.textPrimary) : THEME.textSecondary,
      transition: 'background-color 0.15s',
    }),
    content: {
      flex: 1,
      overflow: 'hidden',
      display: 'flex',
      flexDirection: 'column',
    },
    badge: {
      fontSize: '11px',
      color: THEME.textSecondary,
      opacity: 0.7,
    },
    themeSwitcher: {
      marginLeft: 'auto',
      padding: '4px 8px',
      fontSize: '12px',
      borderRadius: '4px',
      border: `1px solid ${THEME.borderSubtle}`,
      backgroundColor: THEME.surface,
      color: THEME.textSecondary,
      cursor: 'pointer',
    },
  };
}

export default function App() {
  const [activeView, setActiveView] = useState('crawler');
  const [crawlResult, setCrawlResult] = useState(null);
  const [activeThemeSlug, setActiveThemeSlug] = useState(DEFAULT_THEME_SLUG);

  const THEME = getTheme(activeThemeSlug);
  const styles = buildStyles(THEME);

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
        <select
          style={styles.themeSwitcher}
          value={activeThemeSlug}
          onChange={(e) => setActiveThemeSlug(e.target.value)}
          aria-label="Theme"
        >
          {listThemes().map((t) => (
            <option key={t.slug} value={t.slug}>{t.name}</option>
          ))}
        </select>
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

export { getTheme };
