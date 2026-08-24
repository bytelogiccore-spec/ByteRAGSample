<script>
  import { t } from '../lib/i18n.svelte.js';

  let { status = {} } = $props();

  let formattedDate = $derived.by(() => {
    if (!status.last_indexed_at) return '-';
    const d = new Date(status.last_indexed_at * 1000);
    return d.toLocaleTimeString();
  });
</script>

<div class="grid grid-cols-4 gap-4 select-none">
  <!-- Card 1: Indexed Files (Cyan Glow) -->
  <div class="glass-panel rounded-lg p-5 flex flex-col justify-between transition-all relative overflow-hidden group">
    <div class="flex items-center justify-between">
      <span class="text-[11px] font-mono text-[#869397] uppercase tracking-wider">{t('metric_files')}</span>
      <span class="text-xs px-1.5 py-0.5 rounded bg-[#06b6d4]/10 text-[#4cd7f6] font-mono">AST</span>
    </div>
    <div class="my-3">
      <div class="text-3xl font-black font-mono text-[#4cd7f6] tracking-tight">{status.files || 0}</div>
    </div>
    <div class="text-[11px] font-mono text-[#869397]">{t('metric_files_desc')}</div>
  </div>

  <!-- Card 2: Knowledge Nodes (Violet Glow) -->
  <div class="glass-panel rounded-lg p-5 flex flex-col justify-between transition-all relative overflow-hidden group">
    <div class="flex items-center justify-between">
      <span class="text-[11px] font-mono text-[#869397] uppercase tracking-wider">{t('metric_nodes')}</span>
      <span class="text-xs px-1.5 py-0.5 rounded bg-[#8b5cf6]/10 text-[#d0bcff] font-mono">SYMBOLS</span>
    </div>
    <div class="my-3">
      <div class="text-3xl font-black font-mono text-[#d0bcff] tracking-tight">{status.nodes || 0}</div>
    </div>
    <div class="text-[11px] font-mono text-[#869397]">{t('metric_nodes_desc')}</div>
  </div>

  <!-- Card 3: Relation Edges (Emerald Glow) -->
  <div class="glass-panel rounded-lg p-5 flex flex-col justify-between transition-all relative overflow-hidden group">
    <div class="flex items-center justify-between">
      <span class="text-[11px] font-mono text-[#869397] uppercase tracking-wider">{t('metric_edges')}</span>
      <span class="text-xs px-1.5 py-0.5 rounded bg-emerald-500/10 text-emerald-400 font-mono">RELATIONS</span>
    </div>
    <div class="my-3">
      <div class="text-3xl font-black font-mono text-emerald-400 tracking-tight">{status.edges || 0}</div>
    </div>
    <div class="text-[11px] font-mono text-[#869397]">{t('metric_edges_desc')}</div>
  </div>

  <!-- Card 4: 5T Storage Engine Mode (Slate Glow) -->
  <div class="glass-panel rounded-lg p-5 flex flex-col justify-between transition-all relative overflow-hidden group">
    <div class="flex items-center justify-between">
      <span class="text-[11px] font-mono text-[#869397] uppercase tracking-wider">{t('metric_engine')}</span>
      <span class="w-2 h-2 rounded-full bg-emerald-400 shadow-[0_0_6px_#10b981]"></span>
    </div>
    <div class="my-3">
      <div class="text-lg font-bold font-mono text-[#e5e1e4]">5-Tier WAL/WOS</div>
    </div>
    <div class="flex items-center justify-between text-[11px] font-mono text-[#869397]">
      <span>{t('metric_engine_desc')}</span>
      <span class="text-white/40">{formattedDate}</span>
    </div>
  </div>
</div>
