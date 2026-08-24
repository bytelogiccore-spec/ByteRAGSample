<script>
  import { t } from '../lib/i18n.svelte.js';

  let { activeTab = $bindable('tab-overview'), status = {} } = $props();
</script>

<aside class="w-[260px] bg-[#131315] border-r border-white/10 flex flex-col justify-between select-none">
  <div>
    <!-- Brand Header -->
    <div class="p-5 flex items-center gap-3 border-b border-white/10">
      <div class="w-8 h-8 rounded-md bg-gradient-to-br from-[#06b6d4] to-[#8b5cf6] flex items-center justify-center font-black text-sm text-black shadow-[0_0_14px_rgba(6,182,212,0.4)]">
        CO
      </div>
      <div>
        <div class="text-[15px] font-bold text-[#e5e1e4] tracking-tight">CodeOrbit</div>
        <div class="text-[10px] font-mono text-[#4cd7f6] bg-[#06b6d4]/10 px-1.5 py-0.5 rounded">{t('engine_badge')}</div>
      </div>
    </div>

    <!-- Navigation Tabs -->
    <ul class="p-2.5 flex flex-col gap-1.5 list-none">
      <li>
        <button
          onclick={() => activeTab = 'tab-overview'}
          class="w-full flex items-center gap-2.5 px-3.5 py-2.5 rounded-md text-[13px] font-medium transition-all text-left relative {activeTab === 'tab-overview' ? 'bg-[#18181b] text-white border border-white/10' : 'text-[#869397] hover:text-[#e5e1e4] hover:bg-[#18181b]'}"
        >
          {#if activeTab === 'tab-overview'}
            <div class="absolute left-0 top-1.5 bottom-1.5 w-[3px] bg-[#06b6d4] rounded-r shadow-[0_0_8px_#06b6d4]"></div>
          {/if}
          {t('nav_overview')}
        </button>
      </li>
      <li>
        <button
          onclick={() => activeTab = 'tab-docs'}
          class="w-full flex items-center gap-2.5 px-3.5 py-2.5 rounded-md text-[13px] font-medium transition-all text-left relative {activeTab === 'tab-docs' ? 'bg-[#18181b] text-white border border-white/10' : 'text-[#869397] hover:text-[#e5e1e4] hover:bg-[#18181b]'}"
        >
          {#if activeTab === 'tab-docs'}
            <div class="absolute left-0 top-1.5 bottom-1.5 w-[3px] bg-[#06b6d4] rounded-r shadow-[0_0_8px_#06b6d4]"></div>
          {/if}
          {t('nav_docs')}
        </button>
      </li>
      <li>
        <button
          onclick={() => activeTab = 'tab-tests'}
          class="w-full flex items-center gap-2.5 px-3.5 py-2.5 rounded-md text-[13px] font-medium transition-all text-left relative {activeTab === 'tab-tests' ? 'bg-[#18181b] text-white border border-white/10' : 'text-[#869397] hover:text-[#e5e1e4] hover:bg-[#18181b]'}"
        >
          {#if activeTab === 'tab-tests'}
            <div class="absolute left-0 top-1.5 bottom-1.5 w-[3px] bg-[#06b6d4] rounded-r shadow-[0_0_8px_#06b6d4]"></div>
          {/if}
          {t('nav_tests')}
        </button>
      </li>
      <li>
        <button
          onclick={() => activeTab = 'tab-mcp'}
          class="w-full flex items-center gap-2.5 px-3.5 py-2.5 rounded-md text-[13px] font-medium transition-all text-left relative {activeTab === 'tab-mcp' ? 'bg-[#18181b] text-white border border-white/10' : 'text-[#869397] hover:text-[#e5e1e4] hover:bg-[#18181b]'}"
        >
          {#if activeTab === 'tab-mcp'}
            <div class="absolute left-0 top-1.5 bottom-1.5 w-[3px] bg-[#06b6d4] rounded-r shadow-[0_0_8px_#06b6d4]"></div>
          {/if}
          {t('nav_mcp')}
        </button>
      </li>
      <li>
        <button
          onclick={() => activeTab = 'tab-settings'}
          class="w-full flex items-center gap-2.5 px-3.5 py-2.5 rounded-md text-[13px] font-medium transition-all text-left relative {activeTab === 'tab-settings' ? 'bg-[#18181b] text-white border border-white/10' : 'text-[#869397] hover:text-[#e5e1e4] hover:bg-[#18181b]'}"
        >
          {#if activeTab === 'tab-settings'}
            <div class="absolute left-0 top-1.5 bottom-1.5 w-[3px] bg-[#06b6d4] rounded-r shadow-[0_0_8px_#06b6d4]"></div>
          {/if}
          {t('nav_settings')}
        </button>
      </li>
    </ul>
  </div>

  <!-- Sidebar Footer -->
  <div class="p-4 border-t border-white/10 flex flex-col gap-2.5">
    <div class="flex items-center justify-between px-3 py-2 bg-[#18181b] border border-white/10 rounded-md text-[11px] font-mono">
      <div class="flex items-center gap-2">
        <span class="w-[7px] h-[7px] rounded-full {status.indexing ? 'bg-amber-500 shadow-[0_0_6px_#f59e0b]' : 'bg-[#10b981] shadow-[0_0_6px_#10b981]'}"></span>
        <span class="text-[#e5e1e4] font-semibold">{status.indexing ? t('status_indexing') : t('status_ready')}</span>
      </div>
      <span class="text-[#869397]">v0.1.0</span>
    </div>
  </div>
</aside>
