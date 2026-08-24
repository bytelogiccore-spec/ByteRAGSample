<script>
  import { t } from '../lib/i18n.svelte.js';

  let { status = {} } = $props();

  let formattedDate = $derived.by(() => {
    if (!status.last_indexed_at) return '-';
    const d = new Date(status.last_indexed_at * 1000);
    return d.toLocaleTimeString();
  });
</script>

<div class="grid grid-cols-4 gap-3.5 select-none">
  <!-- Card 1: Files -->
  <div class="bg-[#131315] border border-white/10 hover:border-white/20 rounded-md p-4 flex flex-col justify-between transition-all">
    <div class="flex items-center justify-between">
      <span class="text-xs font-mono text-[#869397] uppercase">{t('metric_files')}</span>
      <span class="text-sm">📁</span>
    </div>
    <div class="my-2">
      <div class="text-2xl font-black font-mono text-[#e5e1e4] tracking-tight">{status.files || 0}</div>
    </div>
    <div class="text-[10px] font-mono text-[#869397]">{t('metric_files_desc')}</div>
  </div>

  <!-- Card 2: Knowledge Nodes -->
  <div class="bg-[#131315] border border-white/10 hover:border-[#06b6d4]/40 rounded-md p-4 flex flex-col justify-between transition-all">
    <div class="flex items-center justify-between">
      <span class="text-xs font-mono text-[#869397] uppercase">{t('metric_nodes')}</span>
      <span class="text-sm">🧠</span>
    </div>
    <div class="my-2">
      <div class="text-2xl font-black font-mono text-[#4cd7f6] tracking-tight">{status.nodes || 0}</div>
    </div>
    <div class="text-[10px] font-mono text-[#869397]">{t('metric_nodes_desc')}</div>
  </div>

  <!-- Card 3: Relation Edges -->
  <div class="bg-[#131315] border border-white/10 hover:border-[#8b5cf6]/40 rounded-md p-4 flex flex-col justify-between transition-all">
    <div class="flex items-center justify-between">
      <span class="text-xs font-mono text-[#869397] uppercase">{t('metric_edges')}</span>
      <span class="text-sm">🔗</span>
    </div>
    <div class="my-2">
      <div class="text-2xl font-black font-mono text-[#a78bfa] tracking-tight">{status.edges || 0}</div>
    </div>
    <div class="text-[10px] font-mono text-[#869397]">{t('metric_edges_desc')}</div>
  </div>

  <!-- Card 4: 5T Storage Engine Mode -->
  <div class="bg-[#131315] border border-white/10 hover:border-emerald-500/40 rounded-md p-4 flex flex-col justify-between transition-all">
    <div class="flex items-center justify-between">
      <span class="text-xs font-mono text-[#869397] uppercase">{t('metric_engine')}</span>
      <span class="text-sm">⚡</span>
    </div>
    <div class="my-2 flex items-baseline gap-2">
      <div class="text-lg font-bold font-mono text-emerald-400">5-Tier WAL/WOS</div>
    </div>
    <div class="flex items-center justify-between text-[10px] font-mono text-[#869397]">
      <span>{t('metric_engine_desc')}</span>
      <span class="text-white/40">{formattedDate}</span>
    </div>
  </div>
</div>
