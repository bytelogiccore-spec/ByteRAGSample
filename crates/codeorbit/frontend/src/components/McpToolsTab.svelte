<script>
  import { t } from '../lib/i18n.svelte.js';

  let copyMsg = $state('');

  let tools = [
    { name: 'byterag_query_graph', desc: 'Query nodes and edges via BFS traversal (Depth 1-3)', icon: '🔍' },
    { name: 'byterag_search_symbols', desc: 'Ultra-fast symbol prefix & substring search using Apache Arrow zero-copy memory', icon: '⚡' },
    { name: 'byterag_blast_radius', desc: 'Calculate multi-hop impact radius and reverse dependencies before modifying symbols', icon: '💥' },
    { name: 'byterag_find_path', desc: 'Find shortest dependency path between any two symbols in the knowledge graph', icon: '🛣️' },
    { name: 'byterag_detect_cycles', desc: 'Detect circular dependencies across imports, extends, implements, and calls', icon: '🔄' },
    { name: 'byterag_reindex', desc: 'Trigger incremental indexing on modified workspace source files', icon: '🔁' },
    { name: 'byterag_export_brdb', desc: 'Pack full AST graph + docs + test suites into a single portable .brdb archive', icon: '📦' },
    { name: 'byterag_import_brdb', desc: 'Restore full AST graph and docs instantly from a portable .brdb file', icon: '📥' },
    { name: 'byterag_index_status', desc: 'Inspect current file counts, symbol nodes, dirty flags, and WAL memory state', icon: '📊' },
    { name: 'byterag_get_symbol', desc: 'Retrieve full metadata and location of a specific symbol node', icon: '📌' },
    { name: 'byterag_get_neighbors', desc: 'Get direct 1-hop inbound and outbound connected neighbor symbols', icon: '🌐' },
    { name: 'byterag_list_by_type', desc: 'List symbols filtered by AST type (Struct, Class, Function, Interface)', icon: '📋' },
    { name: 'byterag_read_snippet', desc: 'Read exact code snippet lines for a specific AST node', icon: '📖' },
  ];

  function copyCursorConfig() {
    const config = JSON.stringify({
      mcpServers: {
        codeorbit: {
          command: "d:\\ByteLogicCore\\ByteRAGSample\\target\\debug\\codeorbit.exe",
          args: []
        }
      }
    }, null, 2);

    navigator.clipboard.writeText(config);
    copyMsg = t('mcp_copied');
    setTimeout(() => copyMsg = '', 3000);
  }

  function copyClaudeConfig() {
    const config = JSON.stringify({
      mcpServers: {
        codeorbit: {
          command: "d:\\ByteLogicCore\\ByteRAGSample\\target\\debug\\codeorbit.exe"
        }
      }
    }, null, 2);

    navigator.clipboard.writeText(config);
    copyMsg = t('mcp_copied');
    setTimeout(() => copyMsg = '', 3000);
  }
</script>

<div class="flex-1 flex flex-col gap-4 overflow-hidden select-none">
  <!-- Header & Copy Actions -->
  <div class="bg-[#131315] border border-white/10 rounded-md p-5 flex items-center justify-between">
    <div>
      <div class="text-sm font-bold text-[#e5e1e4] tracking-tight">{t('mcp_header_title')}</div>
      <div class="text-xs font-mono text-[#869397] mt-0.5">
        {t('mcp_header_sub')}
      </div>
    </div>
    <div class="flex items-center gap-2.5">
      {#if copyMsg}
        <span class="text-xs font-mono text-emerald-400 animate-fade-in">{copyMsg}</span>
      {/if}
      <button
        onclick={copyCursorConfig}
        class="px-3 py-1.5 bg-[#18181b] hover:bg-white/10 text-[#4cd7f6] border border-white/10 rounded text-xs font-mono transition-all cursor-pointer"
      >
        {t('mcp_copy_cursor')}
      </button>
      <button
        onclick={copyClaudeConfig}
        class="px-3 py-1.5 bg-[#18181b] hover:bg-white/10 text-[#a78bfa] border border-white/10 rounded text-xs font-mono transition-all cursor-pointer"
      >
        {t('mcp_copy_claude')}
      </button>
      <span class="text-xs font-mono px-2.5 py-1 rounded bg-[#06b6d4]/10 text-[#4cd7f6] border border-[#06b6d4]/20">
        {t('mcp_tool_count')}
      </span>
    </div>
  </div>

  <!-- MCP Tools Grid -->
  <div class="flex-1 overflow-y-auto grid grid-cols-2 gap-3 pr-1">
    {#each tools as tool}
      <div class="p-3.5 bg-[#131315] border border-white/10 hover:border-[#06b6d4]/40 rounded-md flex items-start gap-3 transition-all">
        <span class="text-xl">{tool.icon}</span>
        <div class="flex-1 overflow-hidden">
          <div class="text-xs font-bold font-mono text-[#4cd7f6] truncate">{tool.name}</div>
          <div class="text-[11px] text-[#869397] font-mono mt-1 leading-relaxed">{tool.desc}</div>
        </div>
      </div>
    {/each}
  </div>
</div>
