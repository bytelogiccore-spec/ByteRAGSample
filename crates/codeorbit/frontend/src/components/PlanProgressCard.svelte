<script>
  import { t } from '../lib/i18n.svelte.js';

  let { planStatus = {} } = $props();
</script>

<div class="bg-[#131315] border border-white/10 rounded-md p-4 flex flex-col gap-3 h-full select-none">
  <!-- Header -->
  <div class="flex items-center justify-between border-b border-white/10 pb-2.5">
    <div class="flex items-center gap-2">
      <span class="text-sm">📋</span>
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
        <span class="text-xs font-mono font-bold text-emerald-400">
          {planStatus.progress_percent || 0}%
        </span>
      </div>
    {/if}
  </div>

  {#if planStatus.has_plan && planStatus.milestones && planStatus.milestones.length > 0}
    <!-- Progress Bar -->
    <div class="flex flex-col gap-1">
      <div class="flex justify-between text-[11px] font-mono text-[#869397]">
        <span class="truncate max-w-[200px]">{planStatus.title}</span>
        <span>{planStatus.completed_tasks || 0} / {planStatus.total_tasks || 0}</span>
      </div>
      <div class="w-full h-1.5 bg-[#18181b] rounded-full overflow-hidden border border-white/5">
        <div
          class="h-full bg-gradient-to-r from-[#06b6d4] to-emerald-400 transition-all duration-500 rounded-full"
          style="width: {planStatus.progress_percent || 0}%"
        ></div>
      </div>
    </div>

    <!-- Milestones Checklist -->
    <div class="flex-1 overflow-y-auto flex flex-col gap-1.5 pr-1 max-h-[140px]">
      {#each planStatus.milestones as m}
        <div class="flex items-center gap-2 px-2.5 py-1.5 rounded bg-[#18181b] border border-white/5 text-xs font-mono">
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
