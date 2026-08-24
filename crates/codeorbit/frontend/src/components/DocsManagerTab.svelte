<script>
  import { invokeCommand } from '../lib/tauri.js';

  let docs = $state([]);
  let activeDoc = $state(null);
  let docContent = $state('');
  let isSaving = $state(false);
  let isCreating = $state(false);
  let newDocName = $state('');
  let newDocType = $state('plan');

  async function loadDocs() {
    try {
      const res = await invokeCommand('list_project_docs');
      docs = res || [];
      if (!activeDoc && docs.length > 0) {
        selectDoc(docs[0]);
      }
    } catch (e) {
      console.warn('loadDocs error:', e);
    }
  }

  async function selectDoc(doc) {
    activeDoc = doc;
    try {
      const content = await invokeCommand('read_doc_content', { path: doc.path });
      docContent = content || '';
    } catch (e) {
      alert('문서 읽기 실패: ' + e);
    }
  }

  async function saveDoc() {
    if (!activeDoc) return;
    isSaving = true;
    try {
      await invokeCommand('save_doc_content', { path: activeDoc.path, content: docContent });
      await loadDocs();
    } catch (e) {
      alert('문서 저장 실패: ' + e);
    } finally {
      isSaving = false;
    }
  }

  async function deleteDoc(doc) {
    if (!confirm(`정말 '${doc.name}' 문서를 삭제(정리)하시겠습니까?`)) return;
    try {
      await invokeCommand('delete_doc_file', { path: doc.path });
      activeDoc = null;
      docContent = '';
      await loadDocs();
    } catch (e) {
      alert('문서 삭제 실패: ' + e);
    }
  }

  async function createDoc() {
    const name = newDocName.trim();
    if (!name) return;
    try {
      const path = await invokeCommand('create_doc_file', { name, docType: newDocType, doc_type: newDocType });
      newDocName = '';
      isCreating = false;
      await loadDocs();
      const created = docs.find(d => d.path === path) || { name, path, doc_type: newDocType };
      selectDoc(created);
    } catch (e) {
      alert('문서 생성 실패: ' + e);
    }
  }

  async function openInEditor(path) {
    try {
      await invokeCommand('open_in_ide', { filePath: path, file_path: path, line: 1 });
    } catch (e) {
      alert('에디터 열기 실패: ' + e);
    }
  }

  $effect(() => {
    loadDocs();
  });
</script>

<div class="flex-1 flex gap-4 overflow-hidden select-none">
  <!-- Left: Docs List -->
  <div class="w-72 bg-[#131315] border border-white/10 rounded-md p-4 flex flex-col gap-3">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <span class="text-sm">📚</span>
        <span class="text-xs font-bold text-[#e5e1e4] tracking-tight">프로젝트 문서 관리</span>
      </div>
      <button
        onclick={() => isCreating = !isCreating}
        class="px-2 py-1 bg-[#06b6d4]/15 hover:bg-[#06b6d4]/30 text-[#4cd7f6] border border-[#06b6d4]/30 rounded text-[11px] font-mono transition-all cursor-pointer"
      >
        + 새 문서
      </button>
    </div>

    <!-- Create Form -->
    {#if isCreating}
      <div class="p-2.5 bg-[#18181b] border border-[#06b6d4]/40 rounded flex flex-col gap-2">
        <input
          type="text"
          bind:value={newDocName}
          placeholder="문서명 (예: 03_system_spec)"
          class="bg-[#131315] border border-white/10 rounded px-2.5 py-1 text-xs font-mono text-[#e5e1e4] outline-none"
        />
        <select
          bind:value={newDocType}
          class="bg-[#131315] border border-white/10 rounded px-2 py-1 text-[11px] font-mono text-[#e5e1e4] outline-none"
        >
          <option value="plan">📋 작업 계획서 (Plan)</option>
          <option value="spec">📐 개발 사양서 (Spec)</option>
          <option value="manual">📖 기능 매뉴얼 (Manual)</option>
          <option value="general">📝 일반 마크다운</option>
        </select>
        <div class="flex gap-2">
          <button
            onclick={createDoc}
            class="flex-1 py-1 bg-[#06b6d4] text-black font-bold text-xs rounded"
          >
            생성
          </button>
          <button
            onclick={() => isCreating = false}
            class="px-2 py-1 bg-[#131315] text-[#869397] text-xs rounded border border-white/10"
          >
            취소
          </button>
        </div>
      </div>
    {/if}

    <!-- Docs List Items -->
    <div class="flex-1 overflow-y-auto flex flex-col gap-1.5 pr-1">
      {#each docs as doc}
        <div
          onclick={() => selectDoc(doc)}
          class="p-2.5 rounded border transition-all cursor-pointer flex items-center justify-between group {activeDoc?.path === doc.path ? 'bg-[#201f22] border-[#06b6d4]/50' : 'bg-[#18181b]/70 border-white/5 hover:border-white/20'}"
        >
          <div class="flex flex-col gap-0.5 overflow-hidden">
            <span class="text-xs font-mono font-medium text-[#e5e1e4] truncate group-hover:text-[#4cd7f6]">
              {doc.name}
            </span>
            <div class="flex items-center gap-1.5 text-[10px] font-mono text-[#869397]">
              <span class="px-1 py-0.2 rounded bg-white/5 uppercase text-[9px]">{doc.doc_type}</span>
              <span>{doc.modified_str}</span>
            </div>
          </div>

          <div class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
            <button
              onclick={(e) => { e.stopPropagation(); openInEditor(doc.path); }}
              title="Cursor / IDE에서 열기"
              class="px-1.5 py-0.5 bg-white/10 hover:bg-[#8b5cf6]/30 text-[#d0bcff] rounded text-[10px]"
            >
              IDE
            </button>
            <button
              onclick={(e) => { e.stopPropagation(); deleteDoc(doc); }}
              title="문서 삭제 (완료 후 정리)"
              class="px-1.5 py-0.5 bg-white/10 hover:bg-red-500/30 text-red-400 rounded text-[10px]"
            >
              ✕
            </button>
          </div>
        </div>
      {/each}
    </div>
  </div>

  <!-- Right: Markdown Editor & Viewer -->
  <div class="flex-1 bg-[#131315] border border-white/10 rounded-md p-4 flex flex-col gap-3 overflow-hidden">
    {#if activeDoc}
      <div class="flex items-center justify-between border-b border-white/10 pb-3">
        <div class="flex items-center gap-2.5">
          <span class="text-sm font-mono font-bold text-[#4cd7f6]">{activeDoc.name}</span>
          <span class="text-[11px] font-mono text-[#869397] truncate max-w-[320px]">{activeDoc.path}</span>
        </div>
        <div class="flex items-center gap-2">
          <button
            onclick={() => openInEditor(activeDoc.path)}
            class="px-3 py-1 bg-[#8b5cf6]/20 hover:bg-[#8b5cf6]/35 text-[#d0bcff] border border-[#8b5cf6]/40 text-xs font-mono rounded transition-all cursor-pointer"
          >
            ⚡ Cursor에서 열기
          </button>
          <button
            onclick={saveDoc}
            disabled={isSaving}
            class="px-3.5 py-1 bg-[#06b6d4] hover:bg-[#4cd7f6] text-black font-semibold text-xs font-mono rounded transition-all cursor-pointer shadow-[0_0_10px_rgba(6,182,212,0.3)]"
          >
            {isSaving ? '저장 중...' : '💾 저장하기'}
          </button>
        </div>
      </div>

      <textarea
        bind:value={docContent}
        placeholder="마크다운 문서 내용을 작성하세요..."
        class="flex-1 bg-[#09090b] border border-white/10 focus:border-[#06b6d4] rounded p-3 text-xs font-mono text-[#e5e1e4] outline-none resize-none leading-relaxed"
      ></textarea>
    {:else}
      <div class="m-auto text-xs font-mono text-[#869397]">선택된 문서가 없습니다. 좌측에서 문서를 선택하거나 생성하세요.</div>
    {/if}
  </div>
</div>
