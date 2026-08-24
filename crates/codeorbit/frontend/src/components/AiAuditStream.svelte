<script>
  import { t } from '../lib/i18n.svelte.js';

  let { logs = [] } = $props();
</script>

<div class="glass-panel rounded-lg p-5 flex flex-col gap-3.5 h-full select-none">
  <!-- Header with Live Indicator -->
  <div class="flex items-center justify-between border-b border-white/10 pb-3">
    <div class="flex items-center gap-2.5">
      <div class="w-6 h-6 rounded bg-[#8b5cf6]/10 border border-[#8b5cf6]/30 flex items-center justify-center text-xs">
        ⚡
      </div>
      <div>
        <div class="text-xs font-bold text-[#e5e1e4] tracking-tight">{t('audit_title')}</div>
        <div class="text-[10px] font-mono text-[#869397]">{t('audit_subtitle')}</div>
      </div>
    </div>
    <div class="flex items-center gap-1.5 px-2 py-0.5 rounded bg-emerald-500/10 border border-emerald-500/20 text-[10px] font-mono text-emerald-400">
      <span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-ping"></span>
      LIVE
    </div>
  </div>

  <!-- Audit Event List -->
  {#if logs && logs.length > 0}
    <div class="flex-1 overflow-y-auto flex flex-col gap-1.5 pr-1 max-h-[175px]">
      {#each logs as log}
        <div class="p-2.5 bg-[#18181b]/80 border border-white/5 hover:border-[#06b6d4]/30 rounded-md flex items-center justify-between text-xs font-mono transition-all">
          <div class="flex items-center gap-2 overflow-hidden">
            <span class="text-[#06b6d4] font-bold truncate">{log.tool_name}</span>
            <span class="text-[#869397] truncate">{log.query_target}</span>
          </div>
          <div class="flex items-center gap-2 text-[10px] text-[#869397] shrink-0">
            <span class="text-[#4cd7f6]">{log.duration_ms}ms</span>
            <span class="px-1.5 py-0.5 rounded bg-white/5 border border-white/10 text-emerald-400">{log.status}</span>
          </div>
        </div>
      {/each}
    </div>
  {:else}
    <div class="m-auto text-xs font-mono text-[#869397] py-6 text-center">
      {t('audit_no_logs')}
    </div>
  {/if}
</div>
