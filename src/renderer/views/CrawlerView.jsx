// Deconstructed — Crawler View
// Lets the user enter a target URL, start a crawl, and watch discovered routes appear in real time.

import React, { useState, useEffect, useRef } from 'react';
import RouteCard from '../components/RouteCard.jsx';

const METHOD_COLORS = {
  GET:    '#3B82F6',
  POST:   '#22C55E',
  PUT:    '#F59E0B',
  PATCH:  '#8B5CF6',
  DELETE: '#EF4444',
  HEAD:   '#6B7280',
  OPTIONS:'#14B8A6',
};

export default function CrawlerView({ theme, onCrawlComplete }) {
  const [targetUrl, setTargetUrl] = useState('');
  const [urlError, setUrlError] = useState('');
  const [crawlState, setCrawlState] = useState('idle'); // 'idle'|'running'|'done'|'error'
  const [routes, setRoutes] = useState([]);
  const [assets, setAssets] = useState([]);
  const [progress, setProgress] = useState({ pages: 0, routes: 0, assets: 0 });
  const [log, setLog] = useState([]);
  const cleanupRef = useRef(null);

  useEffect(() => {
    return () => {
      if (cleanupRef.current) cleanupRef.current();
    };
  }, []);

  function validateUrl(url) {
    if (!url.startsWith('https://') && !url.startsWith('http://')) {
      return 'URL must start with https:// or http://';
    }
    try {
      new URL(url);
      return '';
    } catch {
      return 'Invalid URL format';
    }
  }

  async function handleStartCrawl() {
    const error = validateUrl(targetUrl);
    if (error) { setUrlError(error); return; }
    setUrlError('');
    setCrawlState('running');
    setRoutes([]);
    setAssets([]);
    setProgress({ pages: 0, routes: 0, assets: 0 });
    setLog([]);

    const removeListener = window.electronAPI.onCrawlProgress((event) => {
      switch (event.type) {
        case 'page_crawled':
          setProgress((p) => ({ ...p, pages: event.payload.count }));
          setLog((l) => [...l.slice(-49), `📄 ${event.payload.url}`]);
          break;
        case 'route_found':
          setRoutes((r) => [...r, event.payload]);
          setProgress((p) => ({ ...p, routes: p.routes + 1 }));
          break;
        case 'asset_downloaded':
          setAssets((a) => [...a, event.payload]);
          setProgress((p) => ({ ...p, assets: p.assets + 1 }));
          setLog((l) => [...l.slice(-49), `⬇ ${event.payload.filename}`]);
          break;
        case 'error':
          setLog((l) => [...l.slice(-49), `⚠ ${event.payload.message || event.payload.url}`]);
          break;
        case 'complete':
          setCrawlState('done');
          if (onCrawlComplete) {
            onCrawlComplete({
              routes: [],    // populated after final IPC response
              jsAssets: [],  // populated after final IPC response
              sessionId: event.payload.sessionId,
            });
          }
          break;
        default:
          break;
      }
    });

    cleanupRef.current = removeListener;

    try {
      const response = await window.electronAPI.startCrawl({ url: targetUrl });
      if (response.ok) {
        setCrawlState('done');
        if (onCrawlComplete) onCrawlComplete(response.result);
      } else {
        setCrawlState('error');
        setLog((l) => [...l, `❌ ${response.error}`]);
      }
    } catch (err) {
      setCrawlState('error');
      setLog((l) => [...l, `❌ ${err.message}`]);
    } finally {
      removeListener();
    }
  }

  async function handleStopCrawl() {
    await window.electronAPI.stopCrawl('current');
    setCrawlState('idle');
  }

  const s = makeStyles(theme);

  return (
    <div style={s.container}>
      {/* URL Bar */}
      <div style={s.urlBar}>
        <input
          style={{ ...s.input, borderColor: urlError ? theme.danger : theme.borderSubtle }}
          type="text"
          placeholder="https://target-site.example.com"
          value={targetUrl}
          onChange={(e) => { setTargetUrl(e.target.value); setUrlError(''); }}
          onKeyDown={(e) => e.key === 'Enter' && crawlState === 'idle' && handleStartCrawl()}
          disabled={crawlState === 'running'}
        />
        {crawlState !== 'running' ? (
          <button style={s.primaryButton} onClick={handleStartCrawl} disabled={!targetUrl}>
            🕷 Crawl
          </button>
        ) : (
          <button style={{ ...s.primaryButton, backgroundColor: theme.danger }} onClick={handleStopCrawl}>
            ■ Stop
          </button>
        )}
      </div>
      {urlError && <p style={{ color: theme.danger, margin: '4px 20px 0', fontSize: '12px' }}>{urlError}</p>}

      {/* Stats Bar */}
      {crawlState !== 'idle' && (
        <div style={s.statsBar}>
          <span>📄 {progress.pages} pages</span>
          <span>🔗 {progress.routes} routes</span>
          <span>📦 {progress.assets} assets</span>
          {crawlState === 'running' && <span style={{ color: theme.accent }}>● Crawling…</span>}
          {crawlState === 'done' && <span style={{ color: theme.success }}>✓ Complete</span>}
          {crawlState === 'error' && <span style={{ color: theme.danger }}>✗ Error</span>}
        </div>
      )}

      <div style={s.body}>
        {/* Route List */}
        <div style={s.routePanel}>
          <div style={s.panelHeader}>Discovered Routes ({routes.length})</div>
          <div style={s.routeList}>
            {routes.length === 0 && crawlState === 'idle' && (
              <p style={s.emptyState}>Enter a URL above and click Crawl to discover API routes.</p>
            )}
            {routes.map((route) => (
              <RouteCard key={route.id} route={route} theme={theme} methodColors={METHOD_COLORS} />
            ))}
          </div>
        </div>

        {/* Activity Log */}
        <div style={s.logPanel}>
          <div style={s.panelHeader}>Activity Log</div>
          <div style={s.logList}>
            {log.map((line, i) => (
              <div key={i} style={s.logLine}>{line}</div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}

function makeStyles(theme) {
  return {
    container: { display: 'flex', flexDirection: 'column', height: '100%', overflow: 'hidden' },
    urlBar: { display: 'flex', gap: '8px', padding: '16px 20px', alignItems: 'center' },
    input: {
      flex: 1, padding: '8px 12px', borderRadius: '4px',
      border: `1px solid ${theme.borderSubtle}`, backgroundColor: theme.surface,
      color: theme.textPrimary, fontSize: '14px', outline: 'none',
    },
    primaryButton: {
      padding: '8px 20px', borderRadius: '4px', border: 'none',
      backgroundColor: theme.accent, color: '#fff', fontWeight: 600,
      cursor: 'pointer', fontSize: '14px', whiteSpace: 'nowrap',
    },
    statsBar: {
      display: 'flex', gap: '20px', padding: '6px 20px',
      backgroundColor: theme.surface, fontSize: '13px', color: theme.textSecondary,
      borderBottom: `1px solid ${theme.borderSubtle}`,
    },
    body: { display: 'flex', flex: 1, overflow: 'hidden' },
    routePanel: { flex: 2, display: 'flex', flexDirection: 'column', borderRight: `1px solid ${theme.borderSubtle}` },
    logPanel: { flex: 1, display: 'flex', flexDirection: 'column' },
    panelHeader: {
      padding: '8px 16px', fontSize: '12px', fontWeight: 600, textTransform: 'uppercase',
      letterSpacing: '0.08em', color: theme.textSecondary,
      borderBottom: `1px solid ${theme.borderSubtle}`, backgroundColor: theme.surfaceElevated,
    },
    routeList: { flex: 1, overflowY: 'auto', padding: '8px' },
    logList: { flex: 1, overflowY: 'auto', padding: '8px', fontFamily: 'monospace', fontSize: '12px' },
    logLine: { padding: '2px 4px', color: theme.textSecondary, borderBottom: `1px solid ${theme.borderSubtle}20` },
    emptyState: { color: theme.textSecondary, textAlign: 'center', marginTop: '40px', fontSize: '13px' },
  };
}
