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

<!-- Responsive Sidebar: Icon-Rail on small screens (w-16 / 64px), Full Drawer on lg screens (w-64 / 260px) -->
<aside class="w-16 lg:w-64 bg-[#131315] border-r border-white/10 flex flex-col justify-between select-none shrink-0 z-40 transition-all duration-300">
  <div>
    <!-- Stitch Brand Header -->
    <div class="p-3 lg:p-6 flex items-center justify-center lg:justify-start gap-3.5 border-b border-white/10">
      <div class="w-10 h-10 rounded-lg bg-surface-variant flex items-center justify-center border border-white/10 shadow-[0_0_20px_rgba(6,182,212,0.2)] shrink-0">
        <span class="material-symbols-outlined text-[#4cd7f6] text-[22px]">rocket_launch</span>
      </div>
      <div class="hidden lg:block overflow-hidden">
        <h1 class="text-base font-extrabold text-white tracking-tight leading-none truncate">CodeOrbit</h1>
        <span class="text-[10px] font-mono text-[#869397] tracking-wider block mt-1">BYTERAG ENGINE</span>
      </div>
    </div>

    <!-- Navigation List -->
    <nav class="p-2 lg:p-3 space-y-1.5 overflow-y-auto">
      {#each navItems as item}
        <button
          onclick={() => activeTab = item.id}
          title={t(item.labelKey)}
          class="w-full flex items-center justify-center lg:justify-start gap-3 p-2.5 lg:px-3.5 lg:py-2.5 rounded-lg text-[13px] font-medium transition-all text-left relative {activeTab === item.id ? 'bg-[#06b6d4]/10 text-white border-l-2 border-[#06b6d4] font-bold shadow-[0_0_12px_rgba(6,182,212,0.1)]' : 'text-[#869397] hover:text-[#e5e1e4] hover:bg-[#18181b] border-l-2 border-transparent'}"
        >
          <span class="material-symbols-outlined text-[20px] shrink-0 {activeTab === item.id ? 'text-[#4cd7f6]' : 'text-[#869397]'}">{item.icon}</span>
          <span class="hidden lg:inline truncate">{t(item.labelKey)}</span>
        </button>
      {/each}
    </nav>
  </div>

  <!-- Bottom System Telemetry Indicator -->
  <div class="p-2 lg:p-4 border-t border-white/10 flex flex-col gap-2.5">
    <div class="flex items-center justify-center lg:justify-between p-2 lg:px-3 lg:py-2.5 bg-[#18181b] border border-white/10 rounded-md text-[11px] font-mono">
      <div class="flex items-center gap-2">
        <div class="relative w-3 h-3 flex items-center justify-center shrink-0">
          <div class="ambient-glow-cyan"></div>
          <div class="w-1.5 h-1.5 rounded-full {status.indexing ? 'bg-amber-400' : 'bg-[#4cd7f6]'} relative z-10"></div>
        </div>
        <span class="hidden lg:inline text-[#4cd7f6] font-semibold truncate">{status.indexing ? t('status_indexing') : 'MCP READY'}</span>
      </div>
      <span class="hidden lg:inline text-[#869397]">v0.1.0</span>
    </div>
  </div>
</aside>
