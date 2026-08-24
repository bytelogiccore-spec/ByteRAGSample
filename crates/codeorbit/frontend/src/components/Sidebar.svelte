<script>
  import { t } from '../lib/i18n.svelte.js';

  let { activeTab = $bindable('tab-overview'), status = {} } = $props();

  const navItems = [
    { id: 'tab-overview', labelKey: 'nav_overview', icon: 'bolt' },
    { id: 'tab-docs', labelKey: 'nav_docs', icon: 'database' },
    { id: 'tab-tests', labelKey: 'nav_tests', icon: 'verified_user' },
    { id: 'tab-mcp', labelKey: 'nav_mcp', icon: 'extension' },
    { id: 'tab-settings', labelKey: 'nav_settings', icon: 'settings' },
  ];
</script>

<aside class="w-[280px] bg-[#131315] border-r border-white/10 flex flex-col justify-between select-none shrink-0 z-40">
  <div>
    <!-- Stitch Brand Header -->
    <div class="p-6 flex items-center gap-3.5 border-b border-white/10">
      <div class="w-10 h-10 rounded-lg bg-surface-variant flex items-center justify-center border border-white/10 shadow-[0_0_20px_rgba(6,182,212,0.2)]">
        <span class="material-symbols-outlined text-[#4cd7f6] text-[22px]">rocket_launch</span>
      </div>
      <div>
        <h1 class="text-lg font-extrabold text-white tracking-tight leading-none">CodeOrbit</h1>
        <span class="text-[10px] font-mono text-[#869397] tracking-wider block mt-1">BYTERAG ENGINE</span>
      </div>
    </div>

    <!-- Navigation List (Stitch 2px Cyan Border & Background Glow) -->
    <nav class="p-3 space-y-1.5 overflow-y-auto">
      {#each navItems as item}
        <button
          onclick={() => activeTab = item.id}
          class="w-full flex items-center gap-3 px-3.5 py-2.5 rounded-lg text-[13px] font-medium transition-all text-left relative {activeTab === item.id ? 'bg-[#06b6d4]/10 text-white border-l-2 border-[#06b6d4] font-bold shadow-[0_0_12px_rgba(6,182,212,0.1)]' : 'text-[#869397] hover:text-[#e5e1e4] hover:bg-[#18181b] border-l-2 border-transparent'}"
        >
          <span class="material-symbols-outlined text-[20px] {activeTab === item.id ? 'text-[#4cd7f6]' : 'text-[#869397]'}">{item.icon}</span>
          <span>{t(item.labelKey)}</span>
        </button>
      {/each}
    </nav>
  </div>

  <!-- Bottom System Telemetry Indicator -->
  <div class="p-4 border-t border-white/10 flex flex-col gap-2.5">
    <div class="flex items-center justify-between px-3 py-2.5 bg-[#18181b] border border-white/10 rounded-md text-[11px] font-mono">
      <div class="flex items-center gap-2">
        <div class="relative w-3 h-3 flex items-center justify-center">
          <div class="ambient-glow-cyan"></div>
          <div class="w-1.5 h-1.5 rounded-full {status.indexing ? 'bg-amber-400' : 'bg-[#4cd7f6]'} relative z-10"></div>
        </div>
        <span class="text-[#4cd7f6] font-semibold">{status.indexing ? t('status_indexing') : 'MCP READY v0.1.0'}</span>
      </div>
      <span class="text-[#869397]">AST RAG</span>
    </div>
  </div>
</aside>
