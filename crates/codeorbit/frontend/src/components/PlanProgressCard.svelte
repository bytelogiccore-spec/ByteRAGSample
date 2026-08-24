<script>
  import { t } from '../lib/i18n.svelte.js';

  let { planStatus = {} } = $props();
</script>

<div class="glass-panel rounded-lg p-6 flex flex-col h-full select-none">
  <!-- Stitch Header -->
  <div class="flex justify-between items-center mb-5">
    <h3 class="font-mono text-xs font-semibold text-[#e5e1e4] flex items-center gap-2">
      <span class="material-symbols-outlined text-[#4cd7f6] text-[18px]">track_changes</span>
      {t('plan_title')}
    </h3>
    <span class="font-mono text-[11px] text-[#4cd7f6] bg-[#06b6d4]/10 px-2 py-0.5 rounded border border-[#06b6d4]/20">
      {planStatus.progress_percent || 0}%
    </span>
  </div>

  <!-- Stitch Laser Progress Bar -->
  <div class="w-full h-1 bg-[#2a2a2c] rounded-full mb-5 overflow-hidden">
    <div class="h-full bg-[#4cd7f6] relative transition-all duration-500" style="width: {planStatus.progress_percent || 0}%;">
      <div class="absolute top-0 right-0 bottom-0 w-20 bg-gradient-to-r from-transparent to-white/40 animate-pulse"></div>
    </div>
  </div>

  <!-- Stitch Checklist -->
  {#if planStatus.has_plan && planStatus.milestones && planStatus.milestones.length > 0}
    <div class="space-y-2.5 flex-1 overflow-y-auto pr-1 max-h-[170px] custom-scrollbar font-mono">
      {#each planStatus.milestones as m}
        <div class="flex items-start gap-3 p-2.5 rounded {m.completed ? 'bg-[#1c1b1d]/50 border border-[#3d494c]/30' : 'bg-[#06b6d4]/5 border border-[#06b6d4]/30 relative overflow-hidden'}">
          {#if !m.completed}
            <div class="absolute inset-y-0 left-0 w-1 bg-[#4cd7f6]"></div>
          {/if}
          <div class="mt-0.5 {m.completed ? 'text-[#4edea3]' : 'text-[#4cd7f6] animate-pulse'}">
            <span class="material-symbols-outlined text-[16px]">{m.completed ? 'check_circle' : 'radio_button_checked'}</span>
          </div>
          <div class="flex-1 overflow-hidden">
            <div class="text-xs {m.completed ? 'text-[#e5e1e4]' : 'text-[#4cd7f6] font-bold'} truncate">{m.title}</div>
          </div>
        </div>
      {/each}
    </div>
  {:else}
    <div class="m-auto text-xs font-mono text-[#869397] py-8 text-center">
      {t('plan_empty')}
    </div>
  {/if}
</div>
