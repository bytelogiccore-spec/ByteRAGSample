    function getInvoke() {
      if (window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.core.invoke) {
        return window.__TAURI__.core.invoke;
      }
      if (window.__TAURI__ && window.__TAURI__.invoke) {
        return window.__TAURI__.invoke;
      }
      if (window.__TAURI_INTERNALS__ && window.__TAURI_INTERNALS__.invoke) {
        return window.__TAURI_INTERNALS__.invoke;
      }
      return null;
    }

    async function tauriInvoke(cmd, args = {}) {
      const fn = getInvoke();
      if (!fn) {
        throw new Error('Tauri IPC is not ready or not running in Tauri window');
      }
      return await fn(cmd, args);
    }
