<script>
  import { t } from '../lib/i18n.svelte.js';

  let { planStatus = {} } = $props();
</script>

<div class="glass-panel rounded-lg p-5 flex flex-col gap-3.5 h-full select-none">
  <!-- Header -->
  <div class="flex items-center justify-between border-b border-white/10 pb-3">
    <div class="flex items-center gap-2.5">
      <div class="w-6 h-6 rounded bg-[#06b6d4]/10 border border-[#06b6d4]/30 flex items-center justify-center text-xs">
        📋
      </div>
      <div>
        <div class="text-xs font-bold text-[#e5e1e4] tracking-tight">{t('plan_title')}</div>
        <div class="text-[10px] font-mono text-[#869397]">{t('plan_subtitle')}</div>
      </div>
    </div>
    {#if planStatus.has_plan}
      <div class="flex items-center gap-2">
        <span class="text-[10px] font-mono px-2 py-0.5 rounded bg-[#06b6d4]/10 text-[#4cd7f6] border border-[#06b6d4]/20">
          {planStatus.plan_file}
        </span>
        <span class="text-xs font-mono font-bold text-emerald-400 bg-emerald-500/10 px-2 py-0.5 rounded border border-emerald-500/20">
          {planStatus.progress_percent || 0}%
        </span>
      </div>
    {/if}
  </div>

  {#if planStatus.has_plan && planStatus.milestones && planStatus.milestones.length > 0}
    <!-- Progress Bar (Stitch Glowing Pulse) -->
    <div class="flex flex-col gap-1.5">
      <div class="flex justify-between text-[11px] font-mono text-[#869397]">
        <span class="truncate max-w-[220px] text-[#e5e1e4] font-medium">{planStatus.title}</span>
        <span class="font-bold text-[#4cd7f6]">{planStatus.completed_tasks || 0} / {planStatus.total_tasks || 0}</span>
      </div>
      <div class="w-full h-1.5 bg-[#18181b] rounded-full overflow-hidden border border-white/5 relative">
        <div
          class="h-full bg-gradient-to-r from-[#06b6d4] to-emerald-400 transition-all duration-500 rounded-full relative"
          style="width: {planStatus.progress_percent || 0}%"
        >
          <div class="absolute top-0 right-0 bottom-0 w-8 bg-gradient-to-r from-transparent to-white/40 animate-pulse"></div>
        </div>
      </div>
    </div>

    <!-- Milestones Checklist -->
    <div class="flex-1 overflow-y-auto flex flex-col gap-1.5 pr-1 max-h-[140px]">
      {#each planStatus.milestones as m}
        <div class="flex items-center gap-2.5 px-3 py-2 rounded-md bg-[#18181b]/80 border border-white/5 hover:border-white/15 transition-all text-xs font-mono">
          <span class="text-xs {m.completed ? 'text-emerald-400 font-bold' : 'text-[#869397]'}">
            {m.completed ? '✓' : '○'}
          </span>
          <span class="text-xs truncate {m.completed ? 'text-[#e5e1e4]' : 'text-[#869397]'}">
            {m.title}
          </span>
        </div>
      {/each}
    </div>
  {:else}
    <div class="m-auto text-xs font-mono text-[#869397] py-6 text-center">
      {t('plan_empty')}
    </div>
  {/if}
</div>
