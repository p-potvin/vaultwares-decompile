// VaultWares Decompile — API Explorer View
// Postman/Swagger-style panel for replaying intercepted API routes.

import React, { useState } from 'react';
import { groupRoutes } from '../../crawler/route-mapper.js';

const METHOD_COLORS = {
  GET:    '#3B82F6', POST:   '#22C55E', PUT:    '#F59E0B',
  PATCH:  '#8B5CF6', DELETE: '#EF4444', DEFAULT:'#6B7280',
};

export default function ApiExplorerView({ theme, routes = [] }) {
  const [selectedRoute, setSelectedRoute] = useState(null);
  const [response, setResponse] = useState(null);
  const [replaying, setReplaying] = useState(false);
  const [revealHeaders, setRevealHeaders] = useState(false);
  const [editableBody, setEditableBody] = useState('');

  const grouped = groupRoutes(routes);
  const s = makeStyles(theme);

  async function handleReplay() {
    if (!selectedRoute) return;
    setReplaying(true);
    setResponse(null);
    try {
      const result = await window.electronAPI.replayRequest({
        ...selectedRoute,
        body: editableBody ? JSON.parse(editableBody) : selectedRoute.body,
      });
      setResponse(result);
    } catch (err) {
      setResponse({ ok: false, error: err.message });
    } finally {
      setReplaying(false);
    }
  }

  function selectRoute(route) {
    setSelectedRoute(route);
    setEditableBody(route.body ? JSON.stringify(route.body, null, 2) : '');
    setResponse(null);
    setRevealHeaders(false);
  }

  return (
    <div style={s.container}>
      {/* Left: Route Tree */}
      <div style={s.sidebar}>
        <div style={s.sidebarHeader}>Routes ({routes.length})</div>
        <div style={s.routeTree}>
          {Object.keys(grouped).length === 0 ? (
            <p style={s.emptyState}>No routes captured yet.<br/>Run the crawler first.</p>
          ) : (
            Object.entries(grouped).map(([base, groupRoutes]) => (
              <div key={base}>
                <div style={s.groupLabel}>/{base}</div>
                {groupRoutes.map((route) => (
                  <div
                    key={route.id}
                    style={{
                      ...s.routeItem,
                      backgroundColor: selectedRoute?.id === route.id ? theme.surfaceElevated : 'transparent',
                    }}
                    onClick={() => selectRoute(route)}
                  >
                    <span style={{ ...s.methodBadge, backgroundColor: METHOD_COLORS[route.method] || METHOD_COLORS.DEFAULT }}>
                      {route.method}
                    </span>
                    <span style={s.routePattern}>{route.pattern}</span>
                  </div>
                ))}
              </div>
            ))
          )}
        </div>
      </div>

      {/* Right: Request Editor */}
      <div style={s.detail}>
        {!selectedRoute ? (
          <div style={s.emptyDetail}>
            <p style={{ color: theme.textSecondary }}>Select a route to inspect and replay it.</p>
          </div>
        ) : (
          <>
            <div style={s.detailHeader}>
              <span style={{ ...s.methodBadge, fontSize: '15px', backgroundColor: METHOD_COLORS[selectedRoute.method] || METHOD_COLORS.DEFAULT }}>
                {selectedRoute.method}
              </span>
              <code style={s.urlCode}>{selectedRoute.url}</code>
            </div>

            <div style={s.section}>
              <div style={s.sectionTitle}>
                Headers
                <button style={s.toggleButton} onClick={() => setRevealHeaders((v) => !v)}>
                  {revealHeaders ? '🙈 Mask' : '👁 Reveal'}
                </button>
              </div>
              <table style={s.table}>
                <tbody>
                  {Object.entries(selectedRoute.headers || {}).map(([key, value]) => (
                    <tr key={key}>
                      <td style={s.tableKey}>{key}</td>
                      <td style={s.tableValue}>
                        {revealHeaders || !isSensitiveHeader(key) ? value : maskValue(value)}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>

            {editableBody !== '' && (
              <div style={s.section}>
                <div style={s.sectionTitle}>Request Body</div>
                <textarea
                  style={s.bodyEditor}
                  value={editableBody}
                  onChange={(e) => setEditableBody(e.target.value)}
                  spellCheck={false}
                />
              </div>
            )}

            <div style={s.actions}>
              <button style={s.primaryButton} onClick={handleReplay} disabled={replaying}>
                {replaying ? '⏳ Sending…' : '▶ Send Request'}
              </button>
              <button style={s.secondaryButton} onClick={() => alert('Export coming soon')}>
                ⬇ Export
              </button>
            </div>

            {response && (
              <div style={s.responsePanel}>
                <div style={s.sectionTitle}>
                  Response
                  {response.status && (
                    <span style={{ ...s.statusBadge, color: response.status < 400 ? theme.success : theme.danger }}>
                      {response.status} · {response.durationMs}ms
                    </span>
                  )}
                  {!response.ok && <span style={{ color: theme.danger }}> ✗ {response.error}</span>}
                </div>
                <pre style={s.responseBody}>
                  {response.body ? tryFormat(response.body) : '(empty)'}
                </pre>
              </div>
            )}
          </>
        )}
      </div>
    </div>
  );
}

function isSensitiveHeader(key) {
  return /^(authorization|cookie|x-api-key|x-auth-token|x-session)/i.test(key);
}

function maskValue(value) {
  if (!value || value === '[REDACTED]') return '●●●●●●●●';
  const parts = value.split(' ');
  if (parts.length > 1) return `${parts[0]} ●●●●●●●●`;
  return '●●●●●●●●';
}

function tryFormat(text) {
  try { return JSON.stringify(JSON.parse(text), null, 2); }
  catch { return text; }
}

function makeStyles(theme) {
  return {
    container: { display: 'flex', height: '100%', overflow: 'hidden' },
    sidebar: { width: '260px', borderRight: `1px solid ${theme.borderSubtle}`, display: 'flex', flexDirection: 'column', flexShrink: 0 },
    sidebarHeader: { padding: '8px 16px', fontSize: '12px', fontWeight: 600, textTransform: 'uppercase', letterSpacing: '0.08em', color: theme.textSecondary, borderBottom: `1px solid ${theme.borderSubtle}`, backgroundColor: theme.surfaceElevated },
    routeTree: { overflowY: 'auto', flex: 1, padding: '4px 0' },
    groupLabel: { padding: '6px 12px 2px', fontSize: '11px', color: theme.accent, fontWeight: 700, textTransform: 'uppercase' },
    routeItem: { display: 'flex', alignItems: 'center', gap: '8px', padding: '5px 12px', cursor: 'pointer', borderRadius: '3px', margin: '1px 4px' },
    methodBadge: { padding: '2px 6px', borderRadius: '3px', color: '#fff', fontSize: '11px', fontWeight: 700, flexShrink: 0 },
    routePattern: { fontSize: '12px', color: theme.textPrimary, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' },
    detail: { flex: 1, overflowY: 'auto', padding: '16px 20px', display: 'flex', flexDirection: 'column', gap: '16px' },
    emptyDetail: { flex: 1, display: 'flex', alignItems: 'center', justifyContent: 'center' },
    detailHeader: { display: 'flex', alignItems: 'center', gap: '12px' },
    urlCode: { fontSize: '13px', color: theme.textPrimary, fontFamily: 'monospace', wordBreak: 'break-all' },
    section: { display: 'flex', flexDirection: 'column', gap: '8px' },
    sectionTitle: { fontSize: '12px', fontWeight: 700, textTransform: 'uppercase', letterSpacing: '0.06em', color: theme.textSecondary, display: 'flex', alignItems: 'center', gap: '8px' },
    toggleButton: { fontSize: '11px', padding: '2px 8px', borderRadius: '3px', border: `1px solid ${theme.borderSubtle}`, backgroundColor: 'transparent', color: theme.textSecondary, cursor: 'pointer' },
    table: { borderCollapse: 'collapse', width: '100%' },
    tableKey: { padding: '4px 12px 4px 0', fontSize: '12px', color: theme.textSecondary, verticalAlign: 'top', whiteSpace: 'nowrap', fontFamily: 'monospace' },
    tableValue: { padding: '4px 0', fontSize: '12px', color: theme.textPrimary, wordBreak: 'break-all', fontFamily: 'monospace' },
    bodyEditor: { width: '100%', minHeight: '120px', padding: '8px', borderRadius: '4px', border: `1px solid ${theme.borderSubtle}`, backgroundColor: theme.surface, color: theme.textPrimary, fontFamily: 'monospace', fontSize: '12px', resize: 'vertical', boxSizing: 'border-box' },
    actions: { display: 'flex', gap: '8px' },
    primaryButton: { padding: '8px 20px', borderRadius: '4px', border: 'none', backgroundColor: theme.accent, color: '#fff', fontWeight: 600, cursor: 'pointer', fontSize: '14px' },
    secondaryButton: { padding: '8px 16px', borderRadius: '4px', border: `1px solid ${theme.borderSubtle}`, backgroundColor: 'transparent', color: theme.textSecondary, cursor: 'pointer', fontSize: '14px' },
    responsePanel: { display: 'flex', flexDirection: 'column', gap: '8px' },
    statusBadge: { fontSize: '13px', fontWeight: 600 },
    responseBody: { padding: '12px', borderRadius: '4px', backgroundColor: theme.surface, border: `1px solid ${theme.borderSubtle}`, fontSize: '12px', fontFamily: 'monospace', overflow: 'auto', maxHeight: '300px', margin: 0, color: theme.textPrimary },
    emptyState: { padding: '20px', textAlign: 'center', color: theme.textSecondary, fontSize: '13px' },
  };
}
