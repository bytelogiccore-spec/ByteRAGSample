export async function invokeCommand(cmd, args = {}) {
  if (typeof window !== 'undefined') {
    if (window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.core.invoke) {
      return window.__TAURI__.core.invoke(cmd, args);
    }
    if (window.__TAURI__ && window.__TAURI__.invoke) {
      return window.__TAURI__.invoke(cmd, args);
    }
    if (window.__TAURI_INTERNALS__ && window.__TAURI_INTERNALS__.invoke) {
      return window.__TAURI_INTERNALS__.invoke(cmd, args);
    }
  }
  console.warn(`[IPC Fallback] Running ${cmd} outside Tauri`);
  if (cmd === 'get_workspaces') {
    return {
      active_path: 'D:\\ByteLogicCore\\ByteRAGSample',
      workspaces: [
        { name: 'ByteRAGSample', path: 'D:\\ByteLogicCore\\ByteRAGSample' },
        { name: 'codeorbit', path: 'D:\\ByteLogicCore\\ByteRAGSample\\crates\\codeorbit' }
      ]
    };
  }
  if (cmd === 'get_index_status') {
    return {
      target_dir: 'D:\\ByteLogicCore\\ByteRAGSample',
      files: 2273,
      nodes: 99,
      edges: 184,
      indexing: false,
      last_indexed_at: Math.floor(Date.now() / 1000)
    };
  }
  if (cmd === 'search_symbols') {
    return [
      { id: 'fn:main@crates/codeorbit/src/main.rs', name: 'main', node_type: 'function', file_path: 'crates/codeorbit/src/main.rs', language: 'rust', line: 20 },
      { id: 'struct:AppState@crates/codeorbit/src/main.rs', name: 'AppState', node_type: 'struct', file_path: 'crates/codeorbit/src/main.rs', language: 'rust', line: 15 }
    ];
  }
  return null;
}
