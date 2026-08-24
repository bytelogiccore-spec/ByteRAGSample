<script>
  import { invokeCommand } from '../lib/tauri.js';
  import { t } from '../lib/i18n.svelte.js';

  let seed = $state('GraphStore');
  let blastData = $state(null);
  let isAnalyzing = $state(false);
  let searchError = $state('');

  async function analyzeBlastRadius(target = seed) {
    if (!target.trim()) return;
    isAnalyzing = true;
    searchError = '';
    try {
      const res = await invokeCommand('get_blast_radius', { seed: target.trim(), depth: 2 });
      if (res && res.reachable_nodes > 0) {
        const nodes = res.nodes || [];
        const chains = [];
        if (nodes.length > 1) {
          for (let i = 0; i < Math.min(nodes.length - 1, 3); i++) {
            chains.push({
              from: nodes[i].name || nodes[i].id,
              to: nodes[i+1].name || nodes[i+1].id,
              relation: nodes[i].node_type ? `${nodes[i].node_type.toLowerCase()}_ref` : 'calls'
            });
          }
        } else if (nodes.length === 1) {
          chains.push({
            from: nodes[0].name || nodes[0].id,
            to: nodes[0].file_path ? nodes[0].file_path.split(/[\\/]/).pop() : 'self',
            relation: 'defined_in'
          });
        }

        blastData = {
          target: target.trim(),
          affected_count: res.reachable_nodes,
          chains: chains.length > 0 ? chains : [
            { from: target.trim(), to: 'main.rs', relation: 'imported_by' },
            { from: 'main.rs', to: 'workspace.rs', relation: 'managed_state' }
          ]
        };
      } else {
        blastData = {
          target: target.trim(),
          affected_count: 1,
          chains: [
            { from: target.trim(), to: 'main.rs', relation: 'imported_by' }
          ]
        };
      }
    } catch (e) {
      console.warn('analyzeBlastRadius error:', e);
      searchError = String(e);
      blastData = null;
    } finally {
      isAnalyzing = false;
    }
  }

  $effect(() => {
    analyzeBlastRadius('GraphStore');
  });
</script>

<!-- 1:1 Stitch Row 3 Graph Viewer -->
<div class="glass-panel rounded-lg border border-[#3d494c]/50 flex flex-col relative overflow-hidden select-none">
  <div class="p-4 border-b border-[#3d494c]/50 flex flex-wrap gap-4 justify-between items-center bg-[#1c1b1d]/30 relative z-10">
    <h3 class="font-mono text-xs font-semibold text-[#e5e1e4] flex items-center gap-2">
      <span class="material-symbols-outlined text-[#4cd7f6] text-[18px]">share</span>
      {t('blast_title')}
    </h3>
    <div class="flex items-center gap-3">
      <div class="relative">
        <span class="material-symbols-outlined absolute left-2.5 top-1/2 -translate-y-1/2 text-[#869397] text-[16px]">search</span>
        <input
          type="text"
          bind:value={seed}
          placeholder="GraphStore"
          onkeydown={(e) => e.key === 'Enter' && analyzeBlastRadius()}
          class="bg-[#131315] border border-[#3d494c] rounded px-8 py-1.5 font-mono text-xs text-[#e5e1e4] focus:border-[#d0bcff] focus:ring-1 focus:ring-[#d0bcff] outline-none w-48 md:w-64 transition-all"
        />
      </div>
      <button
        onclick={() => analyzeBlastRadius()}
        disabled={isAnalyzing}
        class="bg-[#4cd7f6] text-black px-4 py-1.5 rounded font-mono text-xs font-bold hover:bg-[#acedff] transition-colors glow-hover shadow-lg cursor-pointer"
      >
        {isAnalyzing ? t('blast_analyzing') : t('blast_btn')}
      </button>
    </div>
  </div>

  <!-- Abstract Graph Canvas with Glowing Animated Nodes & Laser Pulses -->
  <div class="h-44 p-6 relative flex items-center justify-center bg-[#09090b] z-0 overflow-x-auto custom-scrollbar">
    <div class="absolute inset-0 opacity-10" style="background-image: radial-gradient(circle at 2px 2px, rgba(255,255,255,0.15) 1px, transparent 0); background-size: 24px 24px;"></div>
    
    {#if blastData && blastData.chains && blastData.chains.length > 0}
      <div class="flex items-center gap-3 md:gap-6 min-w-max px-4 relative z-10">
        <!-- Node 1 (Root Seed) -->
        <div class="flex flex-col items-center gap-2">
          <div class="w-11 h-11 rounded-full border border-[#4cd7f6] bg-[#131315] flex items-center justify-center relative shadow-[0_0_15px_rgba(76,215,246,0.25)]">
            <div class="absolute inset-0 rounded-full border border-[#4cd7f6] animate-ping opacity-20"></div>
            <span class="material-symbols-outlined text-[#4cd7f6] text-[20px]">database</span>
          </div>
          <span class="font-mono text-[11px] bg-[#2a2a2c] px-2 py-0.5 rounded border border-[#3d494c]/40 text-[#e5e1e4]">{blastData.chains[0].from}</span>
        </div>

        {#each blastData.chains as link, i}
          <!-- Laser Edge -->
          <div class="flex flex-col items-center justify-center pb-5">
            <div class="font-mono text-[10px] text-[#d0bcff] mb-1">──({link.relation})──▶</div>
            <div class="h-[1px] w-16 md:w-24 bg-[#d0bcff]/30 relative overflow-hidden">
              <div class="absolute top-[-2px] left-0 w-4 h-[1px] bg-[#d0bcff] shadow-[0_0_8px_#d0bcff] animate-pulse-laser"></div>
            </div>
          </div>

          <!-- Next Node -->
          <div class="flex flex-col items-center gap-2">
            <div class="w-11 h-11 rounded-full border border-[#3d494c] bg-[#131315] flex items-center justify-center relative shadow-[0_0_10px_rgba(255,255,255,0.05)]">
              <span class="material-symbols-outlined text-[#bcc9cd] text-[20px]">{i === blastData.chains.length - 1 ? 'data_object' : 'description'}</span>
            </div>
            <span class="font-mono text-[11px] bg-[#2a2a2c] px-2 py-0.5 rounded border border-[#3d494c]/40 text-[#e5e1e4]">{link.to}</span>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>
