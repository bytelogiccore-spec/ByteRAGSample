<script>
  let { auditLogs = [] } = $props();
</script>

<div class="bg-[#131315] border border-white/10 hover:border-white/20 rounded-md p-5 flex flex-col gap-3 select-none transition-all">
  <div class="flex justify-between items-center">
    <div class="flex items-center gap-2">
      <span class="text-base">⚡</span>
      <span class="text-[13px] font-bold text-[#e5e1e4] tracking-tight">AI 실시간 활동 감사 스트림 (Audit Stream)</span>
    </div>
    <span class="text-[10px] font-mono text-emerald-400 bg-emerald-500/10 px-2 py-0.5 rounded border border-emerald-500/20">
      ● LIVE MONITORING
    </span>
  </div>

  <div class="flex flex-col gap-2 max-h-[220px] overflow-y-auto pr-1">
    {#each auditLogs as log}
      <div class="p-2.5 rounded bg-[#18181b] border border-white/5 hover:border-white/15 transition-all flex flex-col gap-1">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-2">
            <span class="text-[11px] font-mono font-bold text-[#4cd7f6]">{log.tool_name}</span>
            <span class="text-[10px] font-mono text-[#869397]">({log.duration_ms}ms)</span>
          </div>
          <span class="text-[10px] font-mono text-[#869397]">{log.timestamp}</span>
        </div>
        <div class="flex items-center justify-between text-xs font-mono">
          <span class="text-[#e5e1e4] truncate max-w-[280px]">🎯 대상: {log.query_target}</span>
          <span class="text-[10px] text-emerald-400 font-semibold">{log.status}</span>
        </div>
      </div>
    {/each}
  </div>
</div>
