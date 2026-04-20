// Deconstructed — JS Deobfuscator View
// Side-by-side original vs. beautified code viewer with AI rename panel.

import React, { useState } from 'react';

const TRANSFORM_OPTIONS = [
  { key: 'boolean-literals', label: '!0/!1 → true/false' },
  { key: 'hex-literals',     label: '0x... → decimal' },
  { key: 'constant-folding', label: 'Constant folding' },
  { key: 'dead-code',        label: 'Remove dead code' },
  { key: 'sequence-expansion', label: 'Expand sequences' },
];

const CONFIDENCE_THRESHOLD = 0.7;

export default function JsDeobfuscatorView({ theme, assets = [] }) {
  const [selectedAsset, setSelectedAsset] = useState(null);
  const [originalCode, setOriginalCode] = useState('');
  const [beautifiedCode, setBeautifiedCode] = useState('');
  const [selectedTransforms, setSelectedTransforms] = useState(['boolean-literals', 'hex-literals']);
  const [processing, setProcessing] = useState(false);
  const [aiRenaming, setAiRenaming] = useState(false);
  const [proposals, setProposals] = useState([]);
  const [acceptedProposals, setAcceptedProposals] = useState(new Set());
  const [renamedCode, setRenamedCode] = useState('');

  const s = makeStyles(theme);

  async function handleDeobfuscate(asset) {
    setSelectedAsset(asset);
    setProcessing(true);
    setProposals([]);
    setRenamedCode('');
    setAcceptedProposals(new Set());

    try {
      const res = await window.electronAPI.deobfuscate({
        code: asset.code || `// Load from: ${asset.localPath}`,
        transforms: selectedTransforms,
      });

      if (res.ok) {
        setOriginalCode(res.original);
        setBeautifiedCode(res.beautified);
      }
    } catch (err) {
      setBeautifiedCode(`// Error: ${err.message}`);
    } finally {
      setProcessing(false);
    }
  }

  async function handleAiRename() {
    if (!beautifiedCode) return;
    setAiRenaming(true);
    setProposals([]);

    const removeListener = window.electronAPI.onAiRenameProgress((event) => {
      setProposals((prev) => {
        const existing = new Set(prev.map((p) => p.original));
        const newOnes = event.proposals.filter((p) => !existing.has(p.original));
        return [...prev, ...newOnes];
      });
    });

    try {
      const res = await window.electronAPI.aiRename({ code: beautifiedCode });
      if (res.ok) {
        setProposals(res.proposals || []);
        setAcceptedProposals(
          new Set(
            (res.proposals || [])
              .filter((p) => p.confidence >= CONFIDENCE_THRESHOLD)
              .map((p) => p.original)
          )
        );
        setRenamedCode(res.renamedCode || beautifiedCode);
      }
    } catch (err) {
      console.error('[ai-rename]', err);
    } finally {
      setAiRenaming(false);
      removeListener();
    }
  }

  function toggleProposal(original) {
    setAcceptedProposals((prev) => {
      const next = new Set(prev);
      if (next.has(original)) next.delete(original);
      else next.add(original);
      return next;
    });
  }

  const displayCode = renamedCode || beautifiedCode;

  return (
    <div style={s.container}>
      {/* Left: Asset List */}
      <div style={s.assetPanel}>
        <div style={s.panelHeader}>JS Assets ({assets.length})</div>
        <div style={s.assetList}>
          {assets.length === 0 && (
            <p style={s.emptyState}>No JS assets captured.<br/>Run the crawler first.</p>
          )}
          {assets.map((asset) => (
            <div
              key={asset.id}
              style={{
                ...s.assetItem,
                backgroundColor: selectedAsset?.id === asset.id ? theme.surfaceElevated : 'transparent',
              }}
              onClick={() => handleDeobfuscate(asset)}
            >
              <div style={s.assetName}>{asset.filename}</div>
              <div style={s.assetMeta}>
                {formatBytes(asset.sizeBytes)} · {asset.sha256?.slice(0, 8) || ''}
              </div>
            </div>
          ))}
        </div>

        {/* Transform Options */}
        <div style={s.transformPanel}>
          <div style={s.panelHeader}>Transforms</div>
          {TRANSFORM_OPTIONS.map((opt) => (
            <label key={opt.key} style={s.checkboxLabel}>
              <input
                type="checkbox"
                checked={selectedTransforms.includes(opt.key)}
                onChange={(e) => {
                  setSelectedTransforms((prev) =>
                    e.target.checked ? [...prev, opt.key] : prev.filter((k) => k !== opt.key)
                  );
                }}
              />
              <span style={{ marginLeft: '6px', fontSize: '12px', color: theme.textSecondary }}>{opt.label}</span>
            </label>
          ))}
        </div>
      </div>

      {/* Middle: Code Viewer */}
      <div style={s.codePanel}>
        <div style={s.codeToolbar}>
          <span style={{ color: theme.textSecondary, fontSize: '13px' }}>
            {selectedAsset ? selectedAsset.filename : 'No file selected'}
          </span>
          <div style={s.toolbarActions}>
            <button style={s.smallButton} onClick={handleAiRename} disabled={!beautifiedCode || aiRenaming}>
              {aiRenaming ? '⏳ Renaming…' : '🤖 AI Rename'}
            </button>
          </div>
        </div>

        <div style={s.codePanes}>
          <div style={s.codePane}>
            <div style={s.codePaneLabel}>Original (minified)</div>
            <pre style={s.codeBlock}>{processing ? 'Processing…' : originalCode || '// Select an asset from the left panel'}</pre>
          </div>
          <div style={{ ...s.codePane, borderLeft: `1px solid ${theme.borderSubtle}` }}>
            <div style={s.codePaneLabel}>{renamedCode ? 'AI-Renamed' : 'Beautified'}</div>
            <pre style={s.codeBlock}>{processing ? 'Processing…' : displayCode || '// Beautified output appears here'}</pre>
          </div>
        </div>
      </div>

      {/* Right: AI Rename Proposals */}
      {proposals.length > 0 && (
        <div style={s.proposalPanel}>
          <div style={s.panelHeader}>
            Rename Proposals ({proposals.length})
          </div>
          <div style={s.proposalActions}>
            <button style={s.smallButton} onClick={() => setAcceptedProposals(new Set(proposals.map((p) => p.original)))}>
              ✓ All
            </button>
            <button style={s.smallButton} onClick={() =>
              setAcceptedProposals(new Set(proposals.filter((p) => p.confidence >= CONFIDENCE_THRESHOLD).map((p) => p.original)))
            }>
              ✓ High-confidence
            </button>
            <button style={s.smallButton} onClick={() => setAcceptedProposals(new Set())}>
              ✗ None
            </button>
          </div>
          <div style={s.proposalList}>
            {proposals.map((p) => (
              <div
                key={p.original}
                style={{
                  ...s.proposalItem,
                  opacity: acceptedProposals.has(p.original) ? 1 : 0.45,
                }}
                onClick={() => toggleProposal(p.original)}
              >
                <span style={{ color: theme.danger, fontFamily: 'monospace', fontSize: '12px' }}>{p.original}</span>
                <span style={{ color: theme.textSecondary, fontSize: '12px', margin: '0 6px' }}>→</span>
                <span style={{ color: theme.success, fontFamily: 'monospace', fontSize: '12px' }}>{p.proposed}</span>
                <span style={{ marginLeft: 'auto', fontSize: '11px', color: p.confidence >= CONFIDENCE_THRESHOLD ? theme.success : theme.warning }}>
                  {Math.round(p.confidence * 100)}%
                </span>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}

function formatBytes(bytes) {
  if (!bytes) return '0 B';
  const units = ['B', 'KB', 'MB'];
  let i = 0;
  let val = bytes;
  while (val >= 1024 && i < units.length - 1) { val /= 1024; i++; }
  return `${val.toFixed(i > 0 ? 1 : 0)} ${units[i]}`;
}

function makeStyles(theme) {
  return {
    container: { display: 'flex', height: '100%', overflow: 'hidden' },
    assetPanel: { width: '220px', borderRight: `1px solid ${theme.borderSubtle}`, display: 'flex', flexDirection: 'column', flexShrink: 0 },
    panelHeader: { padding: '8px 12px', fontSize: '11px', fontWeight: 700, textTransform: 'uppercase', letterSpacing: '0.08em', color: theme.textSecondary, borderBottom: `1px solid ${theme.borderSubtle}`, backgroundColor: theme.surfaceElevated },
    assetList: { flex: 1, overflowY: 'auto' },
    assetItem: { padding: '8px 12px', cursor: 'pointer', borderBottom: `1px solid ${theme.borderSubtle}20` },
    assetName: { fontSize: '12px', color: theme.textPrimary, wordBreak: 'break-all', fontFamily: 'monospace' },
    assetMeta: { fontSize: '11px', color: theme.textSecondary, marginTop: '2px' },
    transformPanel: { borderTop: `1px solid ${theme.borderSubtle}`, padding: '4px 0' },
    checkboxLabel: { display: 'flex', alignItems: 'center', padding: '4px 12px', cursor: 'pointer' },
    codePanel: { flex: 1, display: 'flex', flexDirection: 'column', overflow: 'hidden' },
    codeToolbar: { display: 'flex', alignItems: 'center', justifyContent: 'space-between', padding: '8px 12px', backgroundColor: theme.surfaceElevated, borderBottom: `1px solid ${theme.borderSubtle}` },
    toolbarActions: { display: 'flex', gap: '8px' },
    smallButton: { padding: '4px 12px', borderRadius: '3px', border: `1px solid ${theme.borderSubtle}`, backgroundColor: 'transparent', color: theme.textSecondary, cursor: 'pointer', fontSize: '12px' },
    codePanes: { display: 'flex', flex: 1, overflow: 'hidden' },
    codePane: { flex: 1, display: 'flex', flexDirection: 'column', overflow: 'hidden' },
    codePaneLabel: { padding: '4px 12px', fontSize: '11px', color: theme.textSecondary, borderBottom: `1px solid ${theme.borderSubtle}`, backgroundColor: theme.surface },
    codeBlock: { flex: 1, overflowY: 'auto', padding: '12px', margin: 0, fontSize: '12px', fontFamily: 'monospace', color: theme.textPrimary, backgroundColor: theme.background, lineHeight: 1.5 },
    proposalPanel: { width: '240px', borderLeft: `1px solid ${theme.borderSubtle}`, display: 'flex', flexDirection: 'column', flexShrink: 0 },
    proposalActions: { display: 'flex', gap: '4px', padding: '6px 8px', borderBottom: `1px solid ${theme.borderSubtle}` },
    proposalList: { flex: 1, overflowY: 'auto' },
    proposalItem: { display: 'flex', alignItems: 'center', padding: '5px 8px', cursor: 'pointer', borderBottom: `1px solid ${theme.borderSubtle}20` },
    emptyState: { padding: '20px', textAlign: 'center', color: theme.textSecondary, fontSize: '13px' },
  };
}
