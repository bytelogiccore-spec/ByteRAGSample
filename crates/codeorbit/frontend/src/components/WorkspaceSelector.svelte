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
      const res = await invokeCommand('export_brdb_file');
      exportSuccessMsg = res ? '✓ Exported' : t('export_brdb_done');
      setTimeout(() => exportSuccessMsg = '', 4000);
    } catch (e) {
      alert('Failed to export .brdb file: ' + e);
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
      alert('Failed to copy prompt: ' + e);
    }
  }

  function toggleLanguage() {
    const nextLang = getLang() === 'ko' ? 'en' : 'ko';
    setLang(nextLang);
  }
</script>

<header class="h-16 border-b border-white/10 bg-[#131315]/95 backdrop-blur-md px-6 flex items-center justify-between z-30 select-none">
  <!-- Left: Workspace Selector with Stitch Search Input Look -->
  <div class="flex items-center gap-4">
    <div class="flex items-center gap-2">
      <span class="material-symbols-outlined text-[#869397] text-[18px]">folder_open</span>
      <span class="text-xs font-mono text-[#869397] uppercase tracking-wider">{t('workspace_label')}:</span>
      <select
        value={activePath}
        onchange={(e) => onSelect(e.target.value)}
        class="bg-[#18181b] border border-white/10 hover:border-white/20 text-[#e5e1e4] text-xs font-mono rounded-md px-3 py-1.5 outline-none cursor-pointer max-w-[320px] truncate transition-all"
      >
        {#each workspaces as ws}
          <option value={ws.path}>{ws.name} ({ws.path})</option>
        {/each}
      </select>
    </div>

    <!-- Add Workspace Button / Form -->
    {#if !isAdding}
      <button
        onclick={() => isAdding = true}
        class="px-2.5 py-1.5 bg-[#18181b] hover:bg-white/10 text-xs font-mono text-[#4cd7f6] rounded-md border border-white/10 transition-all cursor-pointer flex items-center gap-1"
      >
        <span class="material-symbols-outlined text-[14px]">add</span>
        {t('add_workspace')}
      </button>
    {:else}
      <div class="flex items-center gap-1.5 animate-fade-in">
        <input
          type="text"
          bind:value={newPath}
          placeholder="D:\Projects\MyApp"
          class="bg-[#18181b] border border-white/20 text-[#e5e1e4] text-xs font-mono rounded-md px-2.5 py-1.5 outline-none w-56"
        />
        <button
          onclick={handleAdd}
          class="px-3 py-1.5 bg-[#06b6d4] text-black font-bold text-xs font-mono rounded-md cursor-pointer"
        >
          {t('btn_add')}
        </button>
        <button
          onclick={() => { isAdding = false; newPath = ''; }}
          class="px-2.5 py-1.5 bg-[#18181b] text-[#869397] text-xs font-mono rounded-md border border-white/10 cursor-pointer"
        >
          {t('btn_cancel')}
        </button>
      </div>
    {/if}

    {#if workspaces.length > 1}
      <button
        onclick={() => onRemove(activePath)}
        title={t('btn_remove_title')}
        class="px-2 py-1 text-xs font-mono text-red-400/80 hover:text-red-400 hover:bg-red-500/10 rounded border border-transparent hover:border-red-500/20 transition-all cursor-pointer"
      >
        {t('btn_remove')}
      </button>
    {/if}
  </div>

  <!-- Right: Global Actions & Language Toggle -->
  <div class="flex items-center gap-3">
    <!-- Language Toggle Button -->
    <button
      onclick={toggleLanguage}
      title="Switch Language (English / 한국어)"
      class="px-2.5 py-1.5 bg-[#18181b] hover:bg-white/10 text-[#4cd7f6] border border-white/10 rounded-md text-xs font-mono transition-all cursor-pointer flex items-center gap-1.5"
    >
      <span class="material-symbols-outlined text-[15px]">language</span>
      <span class="font-bold">{getLang().toUpperCase()}</span>
    </button>

    <!-- Copy AI Prompt Context -->
    <button
      onclick={handleCopyPrompt}
      title="Copy project GraphRAG knowledge prompt to clipboard"
      class="px-3 py-1.5 bg-gradient-to-r from-[#8b5cf6]/20 to-[#06b6d4]/20 hover:from-[#8b5cf6]/30 hover:to-[#06b6d4]/30 text-[#e5e1e4] hover:text-white border border-[#06b6d4]/30 rounded-md text-xs font-mono font-medium transition-all flex items-center gap-1.5 cursor-pointer shadow-[0_0_12px_rgba(6,182,212,0.15)]"
    >
      <span class="material-symbols-outlined text-[15px] text-[#4cd7f6]">smart_toy</span>
      <span>{copySuccessMsg ? copySuccessMsg : t('prompt_copy')}</span>
    </button>

    <!-- Reindex Button -->
    <button
      onclick={onReindex}
      class="px-3 py-1.5 bg-[#18181b] hover:bg-white/10 text-[#e5e1e4] border border-white/10 rounded-md text-xs font-mono transition-all flex items-center gap-1.5 cursor-pointer"
    >
      <span class="material-symbols-outlined text-[15px]">sync</span>
      <span>{t('reindex')}</span>
    </button>

    <!-- Export .brdb Archive Button -->
    <button
      onclick={handleExportBrdb}
      disabled={isExporting}
      title="Pack all WAL/WOS memories into a single portable .brdb archive file"
      class="px-3.5 py-1.5 bg-[#06b6d4] hover:bg-[#4cd7f6] text-black font-bold text-xs font-mono rounded-md transition-all flex items-center gap-1.5 cursor-pointer shadow-[0_0_12px_rgba(6,182,212,0.3)]"
    >
      <span class="material-symbols-outlined text-[15px]">inventory_2</span>
      <span>{isExporting ? t('export_brdb_packing') : exportSuccessMsg ? exportSuccessMsg : t('export_brdb')}</span>
    </button>

    <!-- Minimize to Tray Button -->
    <button
      onclick={onHideToTray}
      title={t('hide_tray')}
      class="p-1.5 bg-[#18181b] hover:bg-white/10 text-[#869397] hover:text-white border border-white/10 rounded-md transition-all cursor-pointer flex items-center justify-center"
    >
      <span class="material-symbols-outlined text-[18px]">close_fullscreen</span>
    </button>
  </div>
</header>
