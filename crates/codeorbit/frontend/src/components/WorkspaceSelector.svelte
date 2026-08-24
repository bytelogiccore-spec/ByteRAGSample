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

<!-- Responsive TopAppBar with fluid flex-wrap and responsive labels -->
<header class="flex justify-between items-center h-14 lg:h-16 px-3.5 lg:px-6 w-full bg-[#0e0e10] border-b border-[#3d494c]/60 shrink-0 z-50 select-none overflow-hidden">
  <!-- Left: Workspace Selector -->
  <div class="flex items-center gap-2 lg:gap-4 min-w-0">
    <div class="flex items-center gap-1.5 lg:gap-2 min-w-0">
      <span class="text-sm lg:text-lg font-bold text-[#4cd7f6] truncate max-w-[120px] lg:max-w-[220px]">{activeWorkspaceName}</span>
      <select
        value={activePath}
        onchange={(e) => onSelect(e.target.value)}
        class="bg-[#1c1b1d] border border-[#3d494c]/50 text-[#bcc9cd] text-[11px] lg:text-xs font-mono rounded px-2 py-1 outline-none cursor-pointer max-w-[110px] lg:max-w-[180px] truncate"
      >
        {#each workspaces as ws}
          <option value={ws.path}>{ws.name} ({ws.path})</option>
        {/each}
      </select>
    </div>

    {#if !isAdding}
      <button
        onclick={() => isAdding = true}
        class="bg-[#06b6d4]/10 text-[#4cd7f6] border border-[#06b6d4]/30 rounded-lg px-2 lg:px-3 py-1 font-mono text-[10px] lg:text-[11px] font-semibold hover:bg-[#06b6d4]/20 transition-colors flex items-center gap-1 cursor-pointer shrink-0"
      >
        <span class="material-symbols-outlined text-[14px]">add</span>
        <span class="hidden sm:inline">{t('add_workspace')}</span>
      </button>
    {:else}
      <div class="flex items-center gap-1">
        <input
          type="text"
          bind:value={newPath}
          placeholder="D:\Projects\MyApp"
          class="bg-[#18181b] border border-white/20 text-[#e5e1e4] text-xs font-mono rounded px-2 py-1 outline-none w-36 lg:w-52"
        />
        <button
          onclick={handleAdd}
          class="px-2 py-1 bg-[#06b6d4] text-black font-bold text-xs font-mono rounded cursor-pointer"
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

  <!-- Center: Action Links (Condensed on Medium/Small screens) -->
  <div class="flex items-center gap-2 lg:gap-4 shrink-0">
    <nav class="flex items-center gap-1 lg:gap-4 text-xs font-medium text-[#bcc9cd]">
      <button
        onclick={handleCopyPrompt}
        title={t('prompt_copy')}
        class="hover:bg-white/5 transition-colors p-1.5 lg:px-2.5 lg:py-1.5 rounded-md hover:text-[#4cd7f6] cursor-pointer flex items-center gap-1"
      >
        <span class="material-symbols-outlined text-[16px] lg:text-[18px]">smart_toy</span>
        <span class="hidden md:inline">{copySuccessMsg ? copySuccessMsg : t('prompt_copy')}</span>
      </button>
      <button
        onclick={onReindex}
        title={t('reindex')}
        class="hover:bg-white/5 transition-colors p-1.5 lg:px-2.5 lg:py-1.5 rounded-md hover:text-[#4cd7f6] cursor-pointer flex items-center gap-1"
      >
        <span class="material-symbols-outlined text-[16px] lg:text-[18px]">sync</span>
        <span class="hidden md:inline">{t('reindex')}</span>
      </button>
      <button
        onclick={handleExportBrdb}
        disabled={isExporting}
        title={t('export_brdb')}
        class="hover:bg-white/5 transition-colors p-1.5 lg:px-2.5 lg:py-1.5 rounded-md hover:text-[#4cd7f6] cursor-pointer flex items-center gap-1"
      >
        <span class="material-symbols-outlined text-[16px] lg:text-[18px]">inventory_2</span>
        <span class="hidden md:inline">{isExporting ? t('export_brdb_packing') : exportSuccessMsg ? exportSuccessMsg : t('export_brdb')}</span>
      </button>
    </nav>

    <!-- Right: Language Toggle & Window Minimize Controls -->
    <div class="flex items-center gap-1 border-l border-white/10 pl-2 lg:pl-3">
      <button
        onclick={toggleLanguage}
        title="Switch Language"
        class="text-[#bcc9cd] hover:text-[#4cd7f6] transition-colors p-1.5 lg:p-2 rounded hover:bg-white/5 cursor-pointer flex items-center gap-1 text-[11px] lg:text-xs font-mono"
      >
        <span class="material-symbols-outlined text-[16px] lg:text-[18px]">language</span>
        <span class="font-bold">{getLang().toUpperCase()}</span>
      </button>
      <button
        onclick={onHideToTray}
        title={t('hide_tray')}
        class="text-[#bcc9cd] hover:text-[#4cd7f6] transition-colors p-1.5 lg:p-2 rounded hover:bg-white/5 cursor-pointer flex items-center justify-center"
      >
        <span class="material-symbols-outlined text-[16px] lg:text-[18px]">close_fullscreen</span>
      </button>
    </div>
  </div>
</header>
