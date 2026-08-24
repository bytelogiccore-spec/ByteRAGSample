<script>
  import { invokeCommand } from '../lib/tauri.js';

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
      alert('시작프로그램 설정 변경 실패: ' + err);
      autostartEnabled = !val;
    }
  }

  $effect(() => {
    loadAutostart();
  });
</script>

<div class="bg-[#131315] border border-white/10 rounded-md p-5 flex flex-col gap-4 select-none">
  <span class="text-[11px] font-mono text-[#869397] uppercase tracking-wider">SYSTEM & AUTOSTART CONFIGURATION</span>

  <label class="flex items-center gap-3 cursor-pointer text-sm text-[#e5e1e4] p-3 bg-[#18181b] rounded border border-white/10 hover:border-white/20 transition-all">
    <input
      type="checkbox"
      checked={autostartEnabled}
      onchange={handleToggle}
      class="w-4 h-4 accent-[#06b6d4] cursor-pointer"
    />
    <div>
      <span class="font-medium">윈도우 부팅 시 CodeOrbit 자동 실행 (Autostart)</span>
      <p class="text-xs text-[#869397] mt-0.5">컴퓨터를 켤 때 시스템 트레이로 자동 실행되어 에디터(Cursor, Antigravity)가 시작될 때 지식 그래프가 즉시 준비됩니다.</p>
    </div>
  </label>
</div>
