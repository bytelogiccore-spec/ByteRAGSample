<script>
  import { t } from '../lib/i18n.svelte.js';

  let { status = {} } = $props();

  let formattedDate = $derived.by(() => {
    if (!status.last_indexed_at) return '-';
    const d = new Date(status.last_indexed_at * 1000);
    return d.toLocaleTimeString();
  });
</script>

<!-- Responsive Metrics Grid: 1 col on xs, 2 cols on md, 4 cols on xl -->
<div class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-4 gap-3.5 select-none">
  <!-- Card 1: Indexed Files -->
  <div class="glass-panel rounded-lg p-4 lg:p-5 flex flex-col justify-between hover:bg-white/[0.02] transition-all relative overflow-hidden group">
    <div class="absolute top-0 right-0 p-3 opacity-20 group-hover:opacity-40 transition-opacity">
      <span class="material-symbols-outlined text-3xl lg:text-4xl text-[#4cd7f6]">folder_zip</span>
    </div>
    <span class="text-[11px] font-mono text-[#869397] uppercase tracking-wider mb-1.5">{t('metric_files')}</span>
    <div class="font-mono text-2xl lg:text-[28px] font-bold text-[#4cd7f6]">{status.files || 0}</div>
    <div class="text-[11px] font-mono text-[#869397] mt-1 truncate">{t('metric_files_desc')}</div>
  </div>

  <!-- Card 2: Knowledge Symbols -->
  <div class="glass-panel rounded-lg p-4 lg:p-5 flex flex-col justify-between hover:bg-white/[0.02] transition-all relative overflow-hidden group">
    <div class="absolute top-0 right-0 p-3 opacity-20 group-hover:opacity-40 transition-opacity">
      <span class="material-symbols-outlined text-3xl lg:text-4xl text-[#d0bcff]">category</span>
    </div>
    <span class="text-[11px] font-mono text-[#869397] uppercase tracking-wider mb-1.5">{t('metric_nodes')}</span>
    <div class="font-mono text-2xl lg:text-[28px] font-bold text-[#d0bcff]">{status.nodes || 0}</div>
    <div class="text-[11px] font-mono text-[#869397] mt-1 truncate">{t('metric_nodes_desc')}</div>
  </div>

  <!-- Card 3: Dependency Edges -->
  <div class="glass-panel rounded-lg p-4 lg:p-5 flex flex-col justify-between hover:bg-white/[0.02] transition-all relative overflow-hidden group">
    <div class="absolute top-0 right-0 p-3 opacity-20 group-hover:opacity-40 transition-opacity">
      <span class="material-symbols-outlined text-3xl lg:text-4xl text-emerald-400">account_tree</span>
    </div>
    <span class="text-[11px] font-mono text-[#869397] uppercase tracking-wider mb-1.5">{t('metric_edges')}</span>
    <div class="font-mono text-2xl lg:text-[28px] font-bold text-emerald-400">{status.edges || 0}</div>
    <div class="text-[11px] font-mono text-[#869397] mt-1 truncate">{t('metric_edges_desc')}</div>
  </div>

  <!-- Card 4: 5-Tier WAL/WOS Storage Engine Mode -->
  <div class="glass-panel rounded-lg p-4 lg:p-5 flex flex-col justify-between hover:bg-white/[0.02] transition-all relative overflow-hidden group">
    <div class="absolute top-0 right-0 p-3 opacity-20 group-hover:opacity-40 transition-opacity">
      <span class="material-symbols-outlined text-3xl lg:text-4xl text-[#869397]">storage</span>
    </div>
    <span class="text-[11px] font-mono text-[#869397] uppercase tracking-wider mb-1.5">{t('metric_engine')}</span>
    <div class="font-mono text-base lg:text-[18px] font-bold text-[#e5e1e4]">5-Tier WAL/WOS</div>
    <div class="flex items-center justify-between text-[11px] font-mono text-[#869397] mt-1">
      <span class="truncate">{t('metric_engine_desc')}</span>
      <span class="text-white/40">{formattedDate}</span>
    </div>
  </div>
</div>
