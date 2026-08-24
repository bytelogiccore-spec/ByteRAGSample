<script>
  import { t } from '../lib/i18n.svelte.js';

  let { auditLogs = [] } = $props();
</script>

<div class="bg-[#131315] border border-white/10 rounded-md p-4 flex flex-col gap-3 h-full select-none">
  <!-- Header -->
  <div class="flex items-center justify-between border-b border-white/10 pb-2.5">
    <div class="flex items-center gap-2">
      <span class="text-sm">⚡</span>
      <div>
        <div class="text-xs font-bold text-[#e5e1e4] tracking-tight">{t('audit_title')}</div>
        <div class="text-[10px] font-mono text-[#869397]">{t('audit_subtitle')}</div>
      </div>
    </div>
    <div class="flex items-center gap-1.5 text-[10px] font-mono text-emerald-400 bg-emerald-500/10 px-2 py-0.5 rounded border border-emerald-500/20">
      <span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
      LIVE
    </div>
  </div>

  <!-- Audit Event List -->
  <div class="flex-1 overflow-y-auto flex flex-col gap-1.5 pr-1 max-h-[170px]">
    {#if auditLogs.length === 0}
      <div class="m-auto text-xs font-mono text-[#869397] py-6">{t('audit_no_logs')}</div>
    {:else}
      {#each auditLogs as log}
        <div class="px-2.5 py-1.5 rounded bg-[#18181b] border border-white/5 flex items-center justify-between text-xs font-mono">
          <div class="flex items-center gap-2 overflow-hidden">
            <span class="text-[10px] text-[#06b6d4] font-semibold">{log.tool_name}</span>
            <span class="text-[11px] text-[#e5e1e4] truncate max-w-[170px]">{log.query_target}</span>
          </div>
          <div class="flex items-center gap-2 text-[10px] text-[#869397] flex-shrink-0">
            <span class="text-[#4cd7f6]">{log.duration_ms}ms</span>
            <span class="px-1 py-0.2 rounded bg-white/5 text-[9px] text-[#869397]">{log.status}</span>
          </div>
        </div>
      {/each}
    {/if}
  </div>
</div>
