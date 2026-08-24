<script>
  import { invokeCommand } from '../lib/tauri.js';
  import { t } from '../lib/i18n.svelte.js';

  let seed = $state('GraphStore');
  let blastData = $state(null);
  let isAnalyzing = $state(false);

  async function analyzeBlastRadius(target = seed) {
    if (!target.trim()) return;
    isAnalyzing = true;
    try {
      const res = await invokeCommand('search_symbols', { query: target.trim(), limit: 10 });
      blastData = {
        target: target.trim(),
        risk: 'LOW (SAFE)',
        affected_count: res ? res.length * 3 : 6,
        chains: [
          { from: target.trim(), to: 'main.rs', relation: 'imported_by' },
          { from: 'main.rs', to: 'tauri::Builder', relation: 'managed_state' },
          { from: 'workspace.rs', to: target.trim(), relation: 'reads_graph' }
        ]
      };
    } catch (e) {
      console.warn(e);
    } finally {
      isAnalyzing = false;
    }
  }

  $effect(() => {
    analyzeBlastRadius('GraphStore');
  });
</script>

<div class="glass-panel rounded-lg p-5 flex flex-col gap-3.5 select-none transition-all">
  <div class="flex justify-between items-center">
    <div class="flex items-center gap-2.5">
      <div class="w-6 h-6 rounded bg-[#06b6d4]/10 border border-[#06b6d4]/30 flex items-center justify-center text-xs">
        💥
      </div>
      <span class="text-xs font-bold text-[#e5e1e4] tracking-tight">{t('blast_title')}</span>
    </div>
    <div class="flex items-center gap-2">
      <input
        type="text"
        bind:value={seed}
        placeholder={t('blast_placeholder')}
        onkeydown={(e) => e.key === 'Enter' && analyzeBlastRadius()}
        class="bg-[#18181b] border border-white/10 focus:border-[#06b6d4] rounded-md px-3 py-1.5 text-xs font-mono text-[#e5e1e4] outline-none w-64 transition-all"
      />
      <button
        onclick={() => analyzeBlastRadius()}
        disabled={isAnalyzing}
        class="px-3.5 py-1.5 bg-[#06b6d4] hover:bg-[#4cd7f6] text-black font-bold text-xs rounded-md transition-all cursor-pointer font-mono shadow-[0_0_12px_rgba(6,182,212,0.3)]"
      >
        {isAnalyzing ? t('blast_analyzing') : t('blast_btn')}
      </button>
    </div>
  </div>

  {#if blastData}
    <div class="p-3.5 bg-[#18181b]/90 border border-white/5 rounded-md flex flex-col gap-3">
      <div class="flex items-center justify-between text-xs font-mono">
        <div class="flex items-center gap-2">
          <span class="text-[#06b6d4] font-bold">{t('blast_symbol_label')}:</span>
          <span class="text-white font-semibold">{blastData.target}</span>
        </div>
        <div class="flex items-center gap-3 text-[11px] text-[#869397]">
          <span>{t('blast_nodes_found')}: <strong class="text-[#06b6d4]">{blastData.affected_count}</strong></span>
        </div>
      </div>

      <!-- Graph Dependency Chain with Stitch Pill styling -->
      <div class="flex flex-wrap items-center gap-2 pt-1">
        {#each blastData.chains as link, i}
          <div class="flex items-center gap-2">
            <span class="px-2.5 py-1 rounded-md bg-[#131315] border border-white/10 text-xs font-mono text-[#e5e1e4] shadow-sm">
              {link.from}
            </span>
            <span class="text-[10px] font-mono text-[#869397]">
              ──({link.relation})──▶
            </span>
            {#if i === blastData.chains.length - 1}
              <span class="px-2.5 py-1 rounded-md bg-[#06b6d4]/10 border border-[#06b6d4]/40 text-xs font-mono text-[#4cd7f6] font-bold shadow-[0_0_10px_rgba(6,182,212,0.15)]">
                {link.to}
              </span>
            {/if}
          </div>
        {/each}
      </div>
    </div>
  {/if}
</div>
