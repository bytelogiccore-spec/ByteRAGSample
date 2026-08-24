<script>
  import { invokeCommand } from '../lib/tauri.js';

  let {
    workspaces = [],
    activePath = '',
    onSelect = () => {},
    onAdd = () => {},
    onRemove = () => {},
    onReindex = () => {},
    onHideToTray = () => {}
  } = $props();

  let isAdding = $state(false);
  let newPathInput = $state('');

  function handleSelect(e) {
    const val = e.target.value;
    if (val === '__ADD_NEW__') {
      isAdding = true;
    } else {
      onSelect(val);
    }
  }

  async function submitAdd() {
    const p = newPathInput.trim();
    if (!p) return;
    await onAdd(p);
    newPathInput = '';
    isAdding = false;
  }
</script>

<header class="h-14 px-6 border-b border-white/10 flex items-center justify-between bg-[#131315] select-none gap-4">
  <!-- Target Workspace Selector Dropdown -->
  <div class="flex items-center gap-2.5 flex-1 max-w-[620px]">
    <div class="text-[11px] font-mono text-[#869397] whitespace-nowrap uppercase tracking-wider">
      WORKSPACE:
    </div>

    {#if !isAdding}
      <div class="relative flex-1 flex items-center gap-2">
        <select
          value={activePath}
          onchange={handleSelect}
          class="flex-1 bg-[#18181b] border border-white/10 hover:border-white/20 focus:border-[#06b6d4] rounded px-3 py-1.5 text-xs font-mono text-[#e5e1e4] outline-none cursor-pointer truncate transition-all"
        >
          {#each workspaces as ws}
            <option value={ws.path} class="bg-[#18181b] text-white">
              📁 {ws.name} ({ws.path})
            </option>
          {/each}
          <option disabled class="bg-[#18181b] text-gray-500">──────────</option>
          <option value="__ADD_NEW__" class="bg-[#18181b] text-[#4cd7f6] font-semibold">
            + 새 프로젝트 폴더 등록하기...
          </option>
        </select>

        <button
          onclick={() => isAdding = true}
          title="새 워크스페이스 추가"
          class="px-2.5 py-1.5 bg-[#18181b] hover:bg-[#201f22] border border-white/10 hover:border-white/20 rounded text-xs text-[#4cd7f6] transition-all font-mono"
        >
          + 추가
        </button>

        {#if workspaces.length > 1}
          <button
            onclick={() => onRemove(activePath)}
            title="현재 워크스페이스 목록에서 제거"
            class="px-2 py-1.5 bg-[#18181b] hover:bg-red-950/40 border border-white/10 hover:border-red-500/40 rounded text-xs text-red-400 transition-all font-mono"
          >
            ✕
          </button>
        {/if}
      </div>
    {:else}
      <div class="flex-1 flex items-center gap-2">
        <input
          type="text"
          bind:value={newPathInput}
          placeholder="D:\Projects\MyNewApp..."
          onkeydown={(e) => e.key === 'Enter' && submitAdd()}
          class="flex-1 bg-[#18181b] border border-[#06b6d4] rounded px-3 py-1.5 text-xs font-mono text-[#e5e1e4] outline-none"
        />
        <button
          onclick={submitAdd}
          class="px-3 py-1.5 bg-[#06b6d4] hover:bg-[#4cd7f6] text-black font-semibold rounded text-xs transition-all"
        >
          등록
        </button>
        <button
          onclick={() => isAdding = false}
          class="px-2.5 py-1.5 bg-[#18181b] border border-white/10 text-[#869397] hover:text-white rounded text-xs transition-all"
        >
          취소
        </button>
      </div>
    {/if}
  </div>

  <!-- Action Buttons -->
  <div class="flex items-center gap-2.5">
    <button
      onclick={onReindex}
      class="px-3.5 py-1.5 bg-[#06b6d4] hover:bg-[#4cd7f6] text-black font-semibold text-xs rounded transition-all shadow-[0_0_10px_rgba(6,182,212,0.3)] flex items-center gap-1.5 cursor-pointer"
    >
      <span>⚡</span>
      <span>즉시 재인덱싱</span>
    </button>
    <button
      onclick={onHideToTray}
      class="px-3 py-1.5 bg-transparent hover:bg-[#18181b] border border-white/10 hover:border-white/20 text-[#869397] hover:text-[#e5e1e4] text-xs rounded transition-all cursor-pointer"
    >
      트레이로 숨김
    </button>
  </div>
</header>
