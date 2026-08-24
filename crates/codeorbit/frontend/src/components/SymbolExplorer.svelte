<script>
  import { invokeCommand } from '../lib/tauri.js';

  let { status = {} } = $props();

  let query = $state('main');
  let results = $state([]);
  let isSearching = $state(false);
  let errorMsg = $state('');

  let lastIndexTimeString = $derived.by(() => {
    if (!status.last_indexed_at) return '없음';
    const d = new Date(status.last_indexed_at * 1000);
    return d.toLocaleTimeString();
  });

  export async function search(searchQuery = query) {
    const q = searchQuery.trim();
    if (!q) return;
    isSearching = true;
    errorMsg = '';
    try {
      const list = await invokeCommand('search_symbols', { query: q, limit: 50 });
      results = list || [];
    } catch (e) {
      errorMsg = String(e);
      results = [];
    } finally {
      isSearching = false;
    }
  }

  $effect(() => {
    // Initial search
    search('main');
  });
</script>

<div class="flex-1 bg-[#131315] border border-white/10 rounded-md p-5 flex flex-col gap-4 overflow-hidden">
  <div class="flex justify-between items-center select-none">
    <span class="text-[11px] font-mono text-[#869397] uppercase tracking-wider">GRAPH EXPLORER & SYMBOL SEARCH</span>
    <span class="text-[11px] font-mono text-[#869397]">최근 인덱싱: {lastIndexTimeString}</span>
  </div>

  <!-- Search Input Row -->
  <div class="flex gap-2.5">
    <input
      type="text"
      bind:value={query}
      onkeydown={(e) => e.key === 'Enter' && search()}
      placeholder="검색할 심볼 이름, 구조체, 클래스, 함수 (예: UserService, AgentCard, parse_file)"
      class="flex-1 bg-[#18181b] border border-white/10 focus:border-[#06b6d4] focus:shadow-[0_0_10px_rgba(6,182,212,0.2)] rounded px-3.5 py-2 text-[13px] font-mono text-[#e5e1e4] outline-none transition-all"
    />
    <button
      onclick={() => search()}
      class="px-4 py-2 bg-[#06b6d4] hover:bg-[#4cd7f6] text-black font-semibold text-xs rounded transition-all cursor-pointer shadow-[0_0_10px_rgba(6,182,212,0.2)]"
    >
      탐색
    </button>
  </div>

  <!-- Results Box -->
  <div class="flex-1 bg-[#09090b] border border-white/10 rounded p-3 overflow-y-auto min-h-[220px] flex flex-col gap-2">
    {#if isSearching}
      <div class="m-auto text-xs font-mono text-[#06b6d4]">지식 그래프 탐색 중...</div>
    {:else if errorMsg}
      <div class="m-auto text-xs font-mono text-red-400">탐색 실패: {errorMsg}</div>
    {:else if results.length === 0}
      <div class="m-auto text-xs font-mono text-[#869397]">일치하는 심볼이 없습니다.</div>
    {:else}
      {#each results as item}
        <div class="bg-[#18181b] hover:bg-[#201f22] border border-white/10 hover:border-[#06b6d4]/40 rounded p-2.5 px-3.5 flex items-center justify-between transition-all">
          <div>
            <div class="text-[13px] font-semibold text-[#4cd7f6] font-mono">{item.name || item.id}</div>
            <div class="text-[11px] text-[#869397] mt-0.5">
              {item.file_path || ''} {item.line ? `(L${item.line})` : ''}
            </div>
          </div>
          <span class="text-[10px] font-mono px-2 py-0.5 rounded bg-[#8b5cf6]/15 text-[#d0bcff] border border-[#8b5cf6]/30 uppercase">
            {item.node_type || item.language || 'SYM'}
          </span>
        </div>
      {/each}
    {/if}
  </div>
</div>
