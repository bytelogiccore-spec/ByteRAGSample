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

  let activeWorkspaceName = $derived.by(() => {
    const ws = workspaces.find(w => w.path === activePath);
    return ws ? ws.name : 'ByteRAGSample';
  });
</script>

<!-- 1:1 Stitch TopAppBar -->
<header class="flex justify-between items-center h-16 px-6 w-full bg-[#0e0e10] border-b border-[#3d494c]/60 shrink-0 z-50 select-none">
  <!-- Left: Workspace Name & Add Button -->
  <div class="flex items-center gap-4">
    <div class="flex items-center gap-2">
      <span class="text-lg font-bold text-[#4cd7f6]">{activeWorkspaceName}</span>
      <select
        value={activePath}
        onchange={(e) => onSelect(e.target.value)}
        class="bg-[#1c1b1d] border border-[#3d494c]/50 text-[#bcc9cd] text-xs font-mono rounded px-2.5 py-1 outline-none cursor-pointer max-w-[200px] truncate"
      >
        {#each workspaces as ws}
          <option value={ws.path}>{ws.name} ({ws.path})</option>
        {/each}
      </select>
    </div>

    {#if !isAdding}
      <button
        onclick={() => isAdding = true}
        class="bg-[#06b6d4]/10 text-[#4cd7f6] border border-[#06b6d4]/30 rounded-lg px-3 py-1 font-mono text-[11px] font-semibold hover:bg-[#06b6d4]/20 transition-colors flex items-center gap-1 cursor-pointer"
      >
        <span class="material-symbols-outlined text-[14px]">add</span>
        {t('add_workspace')}
      </button>
    {:else}
      <div class="flex items-center gap-1.5">
        <input
          type="text"
          bind:value={newPath}
          placeholder="D:\Projects\MyApp"
          class="bg-[#18181b] border border-white/20 text-[#e5e1e4] text-xs font-mono rounded px-2 py-1 outline-none w-52"
        />
        <button
          onclick={handleAdd}
          class="px-2.5 py-1 bg-[#06b6d4] text-black font-bold text-xs font-mono rounded cursor-pointer"
        >
          {t('btn_add')}
        </button>
        <button
          onclick={() => { isAdding = false; newPath = ''; }}
          class="px-2 py-1 bg-[#18181b] text-[#869397] text-xs font-mono rounded border border-white/10 cursor-pointer"
        >
          {t('btn_cancel')}
        </button>
      </div>
    {/if}
  </div>

  <!-- Center: Action Links (Stitch Exact Styling) -->
  <div class="hidden md:flex items-center gap-6">
    <nav class="flex items-center gap-6 text-xs font-medium text-[#bcc9cd]">
      <button
        onclick={handleCopyPrompt}
        class="hover:bg-white/5 transition-colors px-2.5 py-1.5 rounded-md hover:text-[#4cd7f6] cursor-pointer flex items-center gap-1.5"
      >
        <span class="material-symbols-outlined text-[15px]">smart_toy</span>
        <span>{copySuccessMsg ? copySuccessMsg : t('prompt_copy')}</span>
      </button>
      <button
        onclick={onReindex}
        class="hover:bg-white/5 transition-colors px-2.5 py-1.5 rounded-md hover:text-[#4cd7f6] cursor-pointer flex items-center gap-1.5"
      >
        <span class="material-symbols-outlined text-[15px]">sync</span>
        <span>{t('reindex')}</span>
      </button>
      <button
        onclick={handleExportBrdb}
        disabled={isExporting}
        class="hover:bg-white/5 transition-colors px-2.5 py-1.5 rounded-md hover:text-[#4cd7f6] cursor-pointer flex items-center gap-1.5"
      >
        <span class="material-symbols-outlined text-[15px]">inventory_2</span>
        <span>{isExporting ? t('export_brdb_packing') : exportSuccessMsg ? exportSuccessMsg : t('export_brdb')}</span>
      </button>
    </nav>
  </div>

  <!-- Right: Language Toggle & Window Minimize Controls -->
  <div class="flex items-center gap-2">
    <button
      onclick={toggleLanguage}
      title="Switch Language"
      class="text-[#bcc9cd] hover:text-[#4cd7f6] transition-colors p-2 rounded hover:bg-white/5 cursor-pointer flex items-center gap-1 text-xs font-mono"
    >
      <span class="material-symbols-outlined text-[18px]">language</span>
      <span class="font-bold">{getLang().toUpperCase()}</span>
    </button>
    <button
      onclick={onHideToTray}
      title={t('hide_tray')}
      class="text-[#bcc9cd] hover:text-[#4cd7f6] transition-colors p-2 rounded hover:bg-white/5 cursor-pointer flex items-center justify-center"
    >
      <span class="material-symbols-outlined text-[18px]">close_fullscreen</span>
    </button>
  </div>
</header>
