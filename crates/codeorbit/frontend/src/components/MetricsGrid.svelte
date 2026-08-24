<script>
  let { status = {} } = $props();
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

  <!-- Card 4: Engine -->
  <div class="bg-[#131315] border border-white/10 hover:border-white/20 rounded-md p-4 flex flex-col gap-1.5 transition-all">
    <span class="text-[11px] font-mono text-[#869397] uppercase tracking-wider">STORAGE ENGINE</span>
    <span class="text-xl font-bold text-[#4cd7f6] tracking-tight">ByteRAG 5T</span>
    <span class="text-[11px] font-mono text-[#869397]">.byterag/ (Persistent)</span>
  </div>
</div>
