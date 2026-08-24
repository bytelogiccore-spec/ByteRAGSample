<script>
  import { invokeCommand } from '../lib/tauri.js';
  import { t, getLang, setLang } from '../lib/i18n.svelte.js';

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
  let newPath = $state('');
  let isExporting = $state(false);
  let exportSuccessMsg = $state('');
  let copySuccessMsg = $state('');

  function handleAdd() {
    if (!newPath.trim()) return;
    onAdd(newPath.trim());
    newPath = '';
    isAdding = false;
  }

  async function handleExportBrdb() {
    isExporting = true;
    try {
      const res = await invokeCommand('trigger_export_brdb');
      exportSuccessMsg = res ? `✓ ${res.file_name}` : '✓ .brdb 변환 완료';
      setTimeout(() => exportSuccessMsg = '', 4000);
    } catch (e) {
      alert('단일 .brdb 파일 변환 실패: ' + e);
    } finally {
      isExporting = false;
    }
  }

  async function handleCopyPrompt() {
    try {
      const prompt = await invokeCommand('generate_ai_prompt_context');
      await navigator.clipboard.writeText(prompt);
      copySuccessMsg = t('prompt_copied');
      setTimeout(() => copySuccessMsg = '', 3000);
    } catch (e) {
      alert('프롬프트 복사 실패: ' + e);
    }
  }

  function toggleLanguage() {
    const nextLang = getLang() === 'ko' ? 'en' : 'ko';
    setLang(nextLang);
  }
</script>

<div class="h-14 bg-[#131315] border-b border-white/10 px-5 flex items-center justify-between select-none">
  <!-- Left: Workspace Selector -->
  <div class="flex items-center gap-3">
    <div class="flex items-center gap-2">
      <span class="text-xs font-mono text-[#869397] uppercase tracking-wider">{t('workspace_label')}:</span>
      <select
        value={activePath}
        onchange={(e) => onSelect(e.target.value)}
        class="bg-[#18181b] border border-white/10 hover:border-white/20 text-[#e5e1e4] text-xs font-mono rounded px-2.5 py-1.5 outline-none cursor-pointer max-w-[280px] truncate"
      >
        {#each workspaces as ws}
          <option value={ws.path}>{ws.name} ({ws.path})</option>
        {/each}
      </select>
    </div>

    <!-- Workspace Controls -->
    {#if !isAdding}
      <button
        onclick={() => isAdding = true}
        class="px-2.5 py-1 bg-[#18181b] hover:bg-white/10 text-xs font-mono text-[#4cd7f6] rounded border border-white/10 transition-all cursor-pointer"
      >
        {t('add_workspace')}
      </button>
    {:else}
      <div class="flex items-center gap-1.5 animate-fade-in">
        <input
          type="text"
          bind:value={newPath}
          placeholder="D:\Projects\MyApp"
          class="bg-[#18181b] border border-white/20 text-[#e5e1e4] text-xs font-mono rounded px-2 py-1 outline-none w-52"
        />
        <button
          onclick={handleAdd}
          class="px-2 py-1 bg-[#06b6d4] text-black font-semibold text-xs font-mono rounded cursor-pointer"
        >
          추가
        </button>
        <button
          onclick={() => { isAdding = false; newPath = ''; }}
          class="px-2 py-1 bg-[#18181b] text-[#869397] text-xs font-mono rounded border border-white/10 cursor-pointer"
        >
          취소
        </button>
      </div>
    {/if}

    {#if workspaces.length > 1}
      <button
        onclick={() => onRemove(activePath)}
        title="현재 워크스페이스 제거"
        class="px-2 py-1 text-xs font-mono text-red-400/80 hover:text-red-400 hover:bg-red-500/10 rounded border border-transparent hover:border-red-500/20 transition-all cursor-pointer"
      >
        제거
      </button>
    {/if}
  </div>

  <!-- Right: Global Actions & i18n Language Toggle -->
  <div class="flex items-center gap-2.5">
    <!-- Language Toggle Button -->
    <button
      onclick={toggleLanguage}
      title="Switch Language (한국어 / English)"
      class="px-2.5 py-1.5 bg-[#18181b] hover:bg-white/10 text-[#4cd7f6] border border-white/10 rounded text-xs font-mono transition-all cursor-pointer flex items-center gap-1"
    >
      <span>🌐</span>
      <span class="font-bold">{getLang().toUpperCase()}</span>
    </button>

    <!-- Copy AI Context Prompt Button -->
    <button
      onclick={handleCopyPrompt}
      title="현재 프로젝트의 GraphRAG 지식 아키텍처 프롬프트를 복사하여 AI에게 전달합니다."
      class="px-3 py-1.5 bg-gradient-to-r from-[#8b5cf6]/20 to-[#06b6d4]/20 hover:from-[#8b5cf6]/30 hover:to-[#06b6d4]/30 text-[#e5e1e4] hover:text-white border border-[#06b6d4]/30 rounded text-xs font-mono font-medium transition-all flex items-center gap-1.5 cursor-pointer shadow-[0_0_12px_rgba(6,182,212,0.15)]"
    >
      <span>{copySuccessMsg ? copySuccessMsg : t('prompt_copy')}</span>
    </button>

    <!-- Reindex Button -->
    <button
      onclick={onReindex}
      class="px-3 py-1.5 bg-[#18181b] hover:bg-white/10 text-[#e5e1e4] border border-white/10 rounded text-xs font-mono transition-all flex items-center gap-1.5 cursor-pointer"
    >
      <span>{t('reindex')}</span>
    </button>

    <!-- Export .brdb Button -->
    <button
      onclick={handleExportBrdb}
      disabled={isExporting}
      title="현재 라이브 작업 중인 모든 WAL 및 WOS 메모리를 단 1개의 .brdb 바이너리 파일로 패킹합니다."
      class="px-3 py-1.5 bg-[#06b6d4] hover:bg-[#4cd7f6] text-black font-semibold text-xs font-mono rounded transition-all flex items-center gap-1.5 cursor-pointer shadow-[0_0_10px_rgba(6,182,212,0.3)]"
    >
      <span>{isExporting ? '패킹 중...' : exportSuccessMsg ? exportSuccessMsg : t('export_brdb')}</span>
    </button>

    <!-- Minimize to Tray Button -->
    <button
      onclick={onHideToTray}
      title="백그라운드 시스템 트레이로 숨기기"
      class="px-2.5 py-1.5 bg-[#18181b] hover:bg-white/10 text-[#869397] hover:text-white border border-white/10 rounded text-xs font-mono transition-all cursor-pointer"
    >
      {t('hide_tray')}
    </button>
  </div>
</div>
