<script>
  let { planStatus = {} } = $props();

  let milestones = $derived(planStatus.milestones || []);
  let percent = $derived(planStatus.progress_percent || 0);
</script>

<div class="bg-[#131315] border border-white/10 hover:border-white/20 rounded-md p-5 flex flex-col gap-4 select-none transition-all">
  <div class="flex justify-between items-center">
    <div class="flex items-center gap-2.5">
      <span class="text-base">📋</span>
      <div>
        <div class="text-[13px] font-bold text-[#e5e1e4] tracking-tight">{planStatus.title || 'AI 작업 및 구현 계획'}</div>
        <div class="text-[10px] font-mono text-[#869397]">
          {planStatus.completed_tasks || 0} / {planStatus.total_tasks || 0} 태스크 완료 ({percent}%)
        </div>
      </div>
    </div>
    <span class="text-xs font-mono font-bold px-2 py-0.5 rounded {percent === 100 ? 'bg-emerald-500/15 text-emerald-400 border border-emerald-500/30' : 'bg-[#06b6d4]/15 text-[#4cd7f6] border border-[#06b6d4]/30'}">
      {percent}% 진행 중
    </span>
  </div>

  <!-- Progress Bar -->
  <div class="w-full bg-[#18181b] rounded-full h-2 overflow-hidden border border-white/5">
    <div
      class="bg-gradient-to-r from-[#06b6d4] to-[#8b5cf6] h-full transition-all duration-500 rounded-full"
      style="width: {percent}%"
    ></div>
  </div>

  <!-- Milestones List -->
  <div class="flex flex-col gap-2 max-h-[170px] overflow-y-auto pr-1">
    {#each milestones as m}
      <div class="flex items-center gap-2.5 p-2 rounded bg-[#18181b]/70 border border-white/5 text-xs">
        <span class="{m.completed ? 'text-emerald-400 font-bold' : 'text-[#869397]'}">
          {m.completed ? '✓' : '○'}
        </span>
        <span class="font-mono {m.completed ? 'text-[#e5e1e4]' : 'text-[#869397]'} truncate">
          {m.title}
        </span>
      </div>
    {/each}
  </div>
</div>
