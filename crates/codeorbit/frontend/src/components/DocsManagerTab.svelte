<script>
  import { invokeCommand } from '../lib/tauri.js';
  import { t } from '../lib/i18n.svelte.js';

  let docs = $state([]);
  let activeDoc = $state(null);
  let isSaving = $state(false);
  let isCreating = $state(false);
  let newDocTitle = $state('');
  let newDocType = $state('plan');
  let saveSuccessMsg = $state('');
  let searchQuery = $state('');
  let selectedCategory = $state('ALL');

  async function loadDocs() {
    try {
      const res = await invokeCommand('list_project_docs');
      docs = res || [];
      if (!activeDoc && docs.length > 0) {
        activeDoc = { ...docs[0] };
      }
    } catch (e) {
      console.warn('loadDocs error:', e);
    }
  }

  function selectDoc(doc) {
    activeDoc = { ...doc };
    saveSuccessMsg = '';
  }

  async function saveDoc() {
    if (!activeDoc) return;
    isSaving = true;
    try {
      await invokeCommand('save_doc_to_byterag', { doc: activeDoc });
      saveSuccessMsg = t('docs_saved_msg');
      setTimeout(() => saveSuccessMsg = '', 3000);
      await loadDocs();
    } catch (e) {
      alert('문서 저장 실패: ' + e);
    } finally {
      isSaving = false;
    }
  }

  async function deleteDoc(doc) {
    if (!confirm(`'${doc.title}' 문서를 ByteRAG에서 삭제하시겠습니까?`)) return;
    try {
      await invokeCommand('delete_doc_from_byterag', { id: doc.id });
      activeDoc = null;
      await loadDocs();
    } catch (e) {
      alert('문서 삭제 실패: ' + e);
    }
  }

  async function createDoc() {
    const title = newDocTitle.trim();
    if (!title) return;
    try {
      const newDoc = await invokeCommand('create_doc_in_byterag', { title, docType: newDocType, doc_type: newDocType });
      newDocTitle = '';
      isCreating = false;
      await loadDocs();
      if (newDoc) {
        selectDoc(newDoc);
      }
    } catch (e) {
      alert('문서 생성 실패: ' + e);
    }
  }

  let categories = $derived([
    { key: 'ALL', label: t('docs_cat_all'), icon: '📂' },
    { key: 'plan', label: t('docs_cat_plan'), icon: '📋' },
    { key: 'spec', label: t('docs_cat_spec'), icon: '📐' },
    { key: 'manual', label: t('docs_cat_manual'), icon: '📖' },
    { key: 'general', label: t('docs_cat_general'), icon: '📝' },
  ]);

  let filteredDocs = $derived.by(() => {
    let list = docs;
    if (selectedCategory !== 'ALL') {
      list = list.filter(d => d.doc_type === selectedCategory);
    }
    if (searchQuery.trim()) {
      const q = searchQuery.trim().toLowerCase();
      list = list.filter(d => 
        d.title.toLowerCase().includes(q) || 
        (d.content && d.content.toLowerCase().includes(q))
      );
    }
    return list;
  });

  $effect(() => {
    loadDocs();
  });
</script>

<div class="flex-1 flex gap-4 overflow-hidden select-none">
  <!-- Left: Folders & Tree Explorer -->
  <div class="w-80 bg-[#131315] border border-white/10 rounded-md p-4 flex flex-col gap-3">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <span class="text-sm">🗄️</span>
        <div>
          <div class="text-xs font-bold text-[#e5e1e4] tracking-tight">{t('docs_tree_title')}</div>
          <div class="text-[9px] font-mono text-[#06b6d4]">{t('docs_tree_sub')}</div>
        </div>
      </div>
      <button
        onclick={() => isCreating = !isCreating}
        class="px-2 py-1 bg-[#06b6d4]/15 hover:bg-[#06b6d4]/30 text-[#4cd7f6] border border-[#06b6d4]/30 rounded text-[11px] font-mono transition-all cursor-pointer"
      >
        {t('docs_new_btn')}
      </button>
    </div>

    <!-- Search Box for Docs -->
    <div class="relative">
      <input
        type="text"
        bind:value={searchQuery}
        placeholder={t('docs_search_ph')}
        class="w-full bg-[#18181b] border border-white/10 focus:border-[#06b6d4] rounded px-3 py-1.5 text-xs font-mono text-[#e5e1e4] outline-none transition-all placeholder:text-[#869397]"
      />
      {#if searchQuery}
        <button
          onclick={() => searchQuery = ''}
          class="absolute right-2.5 top-1.5 text-xs text-[#869397] hover:text-white"
        >
          ✕
        </button>
      {/if}
    </div>

    <!-- Folder Category Tree Selector -->
    <div class="flex flex-wrap gap-1 border-b border-white/5 pb-2">
      {#each categories as cat}
        <button
          type="button"
          onclick={() => selectedCategory = cat.key}
          class="px-2 py-1 rounded text-[10px] font-mono transition-all flex items-center gap-1 cursor-pointer {selectedCategory === cat.key ? 'bg-[#06b6d4]/20 text-[#4cd7f6] border border-[#06b6d4]/40 font-bold' : 'bg-[#18181b] text-[#869397] hover:text-[#e5e1e4] border border-white/5'}"
        >
          <span>{cat.icon}</span>
          <span>{cat.label}</span>
        </button>
      {/each}
    </div>

    <!-- Create Form -->
    {#if isCreating}
      <div class="p-2.5 bg-[#18181b] border border-[#06b6d4]/40 rounded flex flex-col gap-2">
        <input
          type="text"
          bind:value={newDocTitle}
          placeholder="Document title (e.g. Auth Architecture Spec)"
          class="bg-[#131315] border border-white/10 rounded px-2.5 py-1 text-xs font-mono text-[#e5e1e4] outline-none"
        />
        <select
          bind:value={newDocType}
          class="bg-[#131315] border border-white/10 rounded px-2 py-1 text-[11px] font-mono text-[#e5e1e4] outline-none"
        >
          <option value="plan">📋 {t('docs_cat_plan')}</option>
          <option value="spec">📐 {t('docs_cat_spec')}</option>
          <option value="manual">📖 {t('docs_cat_manual')}</option>
          <option value="general">📝 {t('docs_cat_general')}</option>
        </select>
        <div class="flex gap-2">
          <button
            onclick={createDoc}
            class="flex-1 py-1 bg-[#06b6d4] text-black font-bold text-xs rounded font-mono cursor-pointer"
          >
            Create in ByteRAG
          </button>
          <button
            onclick={() => isCreating = false}
            class="px-2 py-1 bg-[#131315] text-[#869397] text-xs rounded border border-white/10 font-mono cursor-pointer"
          >
            Cancel
          </button>
        </div>
      </div>
    {/if}

    <!-- Filtered Tree Items -->
    <div class="flex-1 overflow-y-auto flex flex-col gap-1.5 pr-1">
      {#if filteredDocs.length === 0}
        <div class="m-auto text-xs font-mono text-[#869397] py-6">No matching documents.</div>
      {:else}
        {#each filteredDocs as doc}
          <div
            role="button"
            tabindex="0"
            onclick={() => selectDoc(doc)}
            onkeydown={(e) => e.key === 'Enter' && selectDoc(doc)}
            class="w-full text-left p-2.5 rounded border transition-all cursor-pointer flex items-center justify-between group {activeDoc?.id === doc.id ? 'bg-[#201f22] border-[#06b6d4]/50' : 'bg-[#18181b]/70 border-white/5 hover:border-white/20'}"
          >
            <div class="flex flex-col gap-0.5 overflow-hidden">
              <span class="text-xs font-mono font-medium text-[#e5e1e4] truncate group-hover:text-[#4cd7f6]">
                {doc.title}
              </span>
              <div class="flex items-center gap-1.5 text-[10px] font-mono text-[#869397]">
                <span class="px-1 py-0.2 rounded bg-[#06b6d4]/10 text-[#4cd7f6] uppercase text-[9px]">{doc.doc_type}</span>
                <span>ByteRAG DB</span>
              </div>
            </div>

            <button
              type="button"
              onclick={(e) => { e.stopPropagation(); deleteDoc(doc); }}
              title="Delete Document from ByteRAG DB"
              class="px-1.5 py-0.5 bg-white/10 hover:bg-red-500/30 text-red-400 rounded text-[10px] opacity-0 group-hover:opacity-100 transition-opacity"
            >
              ✕
            </button>
          </div>
        {/each}
      {/if}
    </div>
  </div>

  <!-- Right: In-App ByteRAG Editor -->
  <div class="flex-1 bg-[#131315] border border-white/10 rounded-md p-4 flex flex-col gap-3 overflow-hidden">
    {#if activeDoc}
      <div class="flex items-center justify-between border-b border-white/10 pb-3">
        <div class="flex items-center gap-2.5">
          <span class="text-sm font-mono font-bold text-[#4cd7f6]">{activeDoc.title}</span>
          <span class="text-[10px] font-mono px-1.5 py-0.5 rounded bg-white/5 text-[#869397]">Key: {activeDoc.id}</span>
          {#if saveSuccessMsg}
            <span class="text-xs font-mono text-emerald-400 animate-fade-in">{saveSuccessMsg}</span>
          {/if}
        </div>
        <div class="flex items-center gap-2">
          <button
            onclick={saveDoc}
            disabled={isSaving}
            class="px-3.5 py-1 bg-[#06b6d4] hover:bg-[#4cd7f6] text-black font-semibold text-xs font-mono rounded transition-all cursor-pointer shadow-[0_0_10px_rgba(6,182,212,0.3)]"
          >
            {isSaving ? t('docs_saving') : t('docs_save_btn')}
          </button>
        </div>
      </div>

      <textarea
        bind:value={activeDoc.content}
        placeholder="Write Markdown document content to be persisted directly inside ByteRAG DB..."
        class="flex-1 bg-[#09090b] border border-white/10 focus:border-[#06b6d4] rounded p-3.5 text-xs font-mono text-[#e5e1e4] outline-none resize-none leading-relaxed"
      ></textarea>
    {:else}
      <div class="m-auto text-xs font-mono text-[#869397]">{t('docs_no_selected')}</div>
    {/if}
  </div>
</div>
