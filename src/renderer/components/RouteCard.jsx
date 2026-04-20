// Deconstructed — Route Card Component

import React from 'react';

const DEFAULT_METHOD_COLOR = '#6B7280';

export default function RouteCard({ route, theme, methodColors = {} }) {
  const color = methodColors[route.method] || DEFAULT_METHOD_COLOR;

  return (
    <div
      style={{
        display: 'flex',
        alignItems: 'center',
        gap: '10px',
        padding: '8px 10px',
        borderRadius: '4px',
        marginBottom: '4px',
        backgroundColor: theme.surface,
        border: `1px solid ${theme.borderSubtle}`,
        cursor: 'pointer',
      }}
    >
      <span
        style={{
          padding: '2px 8px',
          borderRadius: '3px',
          backgroundColor: color,
          color: '#fff',
          fontSize: '11px',
          fontWeight: 700,
          minWidth: '52px',
          textAlign: 'center',
          flexShrink: 0,
        }}
      >
        {route.method}
      </span>

      <code
        style={{
          flex: 1,
          fontSize: '12px',
          color: theme.textPrimary,
          overflow: 'hidden',
          textOverflow: 'ellipsis',
          whiteSpace: 'nowrap',
          fontFamily: 'monospace',
        }}
      >
        {route.pattern}
      </code>

      <span
        style={{
          fontSize: '11px',
          color: theme.textSecondary,
          flexShrink: 0,
        }}
      >
        ×{route.interceptedCount}
      </span>
    </div>
  );
}
