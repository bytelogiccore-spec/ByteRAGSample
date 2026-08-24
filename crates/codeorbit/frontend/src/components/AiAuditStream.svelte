<script>
  import { t } from '../lib/i18n.svelte.js';

  let { logs = [] } = $props();
</script>

<div class="glass-panel rounded-lg p-0 flex flex-col overflow-hidden border border-white/10 h-full select-none">
  <!-- Terminal Header Bar with Traffic Lights -->
  <div class="p-4 border-b border-white/10 bg-[#18181b]/60 flex justify-between items-center">
    <div class="flex items-center gap-2">
      <span class="material-symbols-outlined text-[#d0bcff] text-[18px]">terminal</span>
      <span class="font-mono text-xs font-semibold text-[#e5e1e4]">{t('audit_title')}</span>
    </div>
    <div class="flex items-center gap-2">
      <div class="flex gap-1">
        <div class="w-2 h-2 rounded-full bg-red-500/80"></div>
        <div class="w-2 h-2 rounded-full bg-amber-500/80"></div>
        <div class="w-2 h-2 rounded-full bg-emerald-500/80"></div>
      </div>
      <span class="text-[10px] font-mono text-emerald-400 bg-emerald-500/10 px-2 py-0.5 rounded border border-emerald-500/20 ml-2">
        LIVE STREAM
      </span>
    </div>
  </div>

  <!-- Terminal Event Log Body -->
  {#if logs && logs.length > 0}
    <div class="p-4 space-y-2 flex-1 overflow-y-auto font-mono text-xs max-h-[175px] bg-[#09090b]/80">
      {#each logs as log}
        <div class="flex items-start justify-between p-2 rounded bg-white/[0.02] border border-white/5 hover:border-[#06b6d4]/30 transition-colors">
          <div class="flex items-center gap-2 overflow-hidden">
            <span class="text-[#869397] text-[10px]">{log.timestamp}</span>
            <span class="text-[#4cd7f6] font-bold truncate">{log.tool_name}</span>
            <span class="text-[#e5e1e4] truncate text-[11px]">{log.query_target}</span>
          </div>
          <div class="flex items-center gap-2 text-[10px] shrink-0">
            <span class="text-[#d0bcff]">{log.duration_ms}ms</span>
            <span class="px-1.5 py-0.5 rounded bg-emerald-500/10 text-emerald-400 border border-emerald-500/20">{log.status}</span>
          </div>
        </div>
      {/each}
    </div>
  {:else}
    <div class="m-auto text-xs font-mono text-[#869397] py-10 text-center">
      {t('audit_no_logs')}
    </div>
  {/if}
</div>
