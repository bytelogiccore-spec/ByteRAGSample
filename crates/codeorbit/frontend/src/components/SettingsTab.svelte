<script>
  import { invokeCommand } from '../lib/tauri.js';
  import { t } from '../lib/i18n.svelte.js';

  let autostartEnabled = $state(false);

  async function loadAutostart() {
    try {
      autostartEnabled = await invokeCommand('is_autostart_enabled');
    } catch (e) {}
  }

  async function handleToggle(e) {
    const val = e.target.checked;
    try {
      await invokeCommand('toggle_autostart', { enabled: val });
      autostartEnabled = val;
    } catch (err) {
      alert('Autostart configuration error: ' + err);
      autostartEnabled = !val;
    }
  }

  $effect(() => {
    loadAutostart();
  });
</script>

<div class="bg-[#131315] border border-white/10 rounded-md p-5 flex flex-col gap-4 select-none">
  <span class="text-[11px] font-mono text-[#869397] uppercase tracking-wider">{t('settings_header')}</span>

  <label class="flex items-center gap-3 cursor-pointer text-sm text-[#e5e1e4] p-3 bg-[#18181b] rounded border border-white/10 hover:border-white/20 transition-all">
    <input
      type="checkbox"
      checked={autostartEnabled}
      onchange={handleToggle}
      class="w-4 h-4 accent-[#06b6d4] cursor-pointer"
    />
    <div>
      <span class="font-medium">{t('settings_autostart_title')}</span>
      <p class="text-xs text-[#869397] mt-0.5">{t('settings_autostart_desc')}</p>
    </div>
  </label>
</div>
