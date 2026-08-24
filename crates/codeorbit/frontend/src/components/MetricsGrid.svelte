<script>
  import { invokeCommand } from '../lib/tauri.js';

  let { status = {} } = $props();
  let isPacking = $state(false);
  let packMessage = $state('');

  async function handleForcePack() {
    isPacking = true;
    packMessage = '';
    try {
      const res = await invokeCommand('export_brdb_file', { path: null });
      packMessage = res || '성공적으로 단일 .brdb 파일로 압축 변환되었습니다.';
      setTimeout(() => packMessage = '', 4000);
    } catch (e) {
      alert('단일 파일 압축 변환 실패: ' + e);
    } finally {
      isPacking = false;
    }
  }
</script>

<div class="grid grid-cols-4 gap-3.5 select-none">
  <!-- Card 1: Files -->
  <div class="bg-[#131315] border border-white/10 hover:border-white/20 rounded-md p-4 flex flex-col gap-1.5 transition-all">
    <span class="text-[11px] font-mono text-[#869397] uppercase tracking-wider">INDEXED FILES</span>
    <span class="text-2xl font-bold text-[#e5e1e4] tracking-tight">{(status.files || 0).toLocaleString()}</span>
    <span class="text-[11px] font-mono text-[#869397]">Rust · C++ · C# · TS · Py</span>
  </div>

  <!-- Card 2: Knowledge Nodes -->
  <div class="bg-[#131315] border border-white/10 hover:border-white/20 rounded-md p-4 flex flex-col gap-1.5 transition-all">
    <span class="text-[11px] font-mono text-[#869397] uppercase tracking-wider">KNOWLEDGE NODES</span>
    <span class="text-2xl font-bold text-[#e5e1e4] tracking-tight">{(status.nodes || 0).toLocaleString()}</span>
    <span class="text-[11px] font-mono text-[#869397]">Classes, Structs, Funcs</span>
  </div>

  <!-- Card 3: Relation Edges -->
  <div class="bg-[#131315] border border-white/10 hover:border-white/20 rounded-md p-4 flex flex-col gap-1.5 transition-all">
    <span class="text-[11px] font-mono text-[#869397] uppercase tracking-wider">RELATION EDGES</span>
    <span class="text-2xl font-bold text-[#e5e1e4] tracking-tight">{(status.edges || 0).toLocaleString()}</span>
    <span class="text-[11px] font-mono text-[#869397]">Imports, Inherits, Calls</span>
  </div>

  <!-- Card 4: Engine + Force Pack Button -->
  <div class="bg-[#131315] border border-white/10 hover:border-white/20 rounded-md p-4 flex flex-col justify-between transition-all relative overflow-hidden group">
    <div class="flex justify-between items-start">
      <span class="text-[11px] font-mono text-[#869397] uppercase tracking-wider">STORAGE ENGINE</span>
      <button
        onclick={handleForcePack}
        disabled={isPacking}
        title="여러 개의 WAL/WOS 캐시 파일을 단 1개의 고성능 .brdb 포터블 바이너리 파일로 즉시 압축 변환합니다."
        class="text-[10px] font-mono px-2 py-0.5 rounded bg-[#06b6d4]/15 hover:bg-[#06b6d4]/30 text-[#4cd7f6] border border-[#06b6d4]/30 transition-all cursor-pointer disabled:opacity-50"
      >
        {isPacking ? '압축 중...' : '📦 단일 .brdb 변환'}
      </button>
    </div>

    <div>
      <span class="text-xl font-bold text-[#4cd7f6] tracking-tight">ByteRAG 5T</span>
      <div class="text-[11px] font-mono text-[#869397] mt-0.5">.byterag/graph.brdb</div>
    </div>

    {#if packMessage}
      <div class="absolute inset-0 bg-[#131315]/95 p-2 flex items-center justify-center text-center text-[11px] font-mono text-[#10b981] animate-fade-in">
        {packMessage}
      </div>
    {/if}
  </div>
</div>
