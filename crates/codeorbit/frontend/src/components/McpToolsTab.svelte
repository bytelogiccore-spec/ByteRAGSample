<script>
  import { t } from '../lib/i18n.svelte.js';

  let copyMsg = $state('');

  let tools = [
    { name: 'byterag_query_graph', descKey: 'tool_desc_query_graph', icon: '🔍' },
    { name: 'byterag_search_symbols', descKey: 'tool_desc_search_symbols', icon: '⚡' },
    { name: 'byterag_blast_radius', descKey: 'tool_desc_blast_radius', icon: '💥' },
    { name: 'byterag_find_path', descKey: 'tool_desc_find_path', icon: '🛣️' },
    { name: 'byterag_detect_cycles', descKey: 'tool_desc_detect_cycles', icon: '🔄' },
    { name: 'byterag_reindex', descKey: 'tool_desc_reindex', icon: '🔁' },
    { name: 'byterag_export_brdb', descKey: 'tool_desc_export_brdb', icon: '📦' },
    { name: 'byterag_import_brdb', descKey: 'tool_desc_import_brdb', icon: '📥' },
    { name: 'byterag_index_status', descKey: 'tool_desc_index_status', icon: '📊' },
    { name: 'byterag_get_symbol', descKey: 'tool_desc_get_symbol', icon: '📌' },
    { name: 'byterag_get_neighbors', descKey: 'tool_desc_get_neighbors', icon: '🌐' },
    { name: 'byterag_list_by_type', descKey: 'tool_desc_list_by_type', icon: '📋' },
    { name: 'byterag_read_snippet', descKey: 'tool_desc_read_snippet', icon: '📖' },
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

  <!-- MCP Tools Grid with Live Reactive i18n Tool Descriptions -->
  <div class="flex-1 overflow-y-auto grid grid-cols-2 gap-3 pr-1 custom-scrollbar">
    {#each tools as tool}
      <div class="p-3.5 bg-[#131315] border border-white/10 hover:border-[#06b6d4]/40 rounded-md flex items-start gap-3 transition-all">
        <span class="text-xl">{tool.icon}</span>
        <div class="flex-1 overflow-hidden">
          <div class="text-xs font-bold font-mono text-[#4cd7f6] truncate">{tool.name}</div>
          <div class="text-[11px] text-[#869397] font-mono mt-1 leading-relaxed">{t(tool.descKey)}</div>
        </div>
      </div>
    {/each}
  </div>
</div>
