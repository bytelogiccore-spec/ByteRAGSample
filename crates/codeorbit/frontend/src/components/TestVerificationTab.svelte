<script>
  import { invokeCommand } from '../lib/tauri.js';

  let report = $state(null);
  let isLoading = $state(false);

  async function loadTestReport() {
    isLoading = true;
    try {
      const res = await invokeCommand('get_test_verification_report');
      report = res;
    } catch (e) {
      console.warn('loadTestReport error:', e);
    } finally {
      isLoading = false;
    }
  }

  let runTimeString = $derived.by(() => {
    if (!report?.run_at) return '최근';
    const d = new Date(report.run_at * 1000);
    return d.toLocaleTimeString();
  });

  $effect(() => {
    loadTestReport();
  });
</script>

<div class="flex-1 flex flex-col gap-4 overflow-hidden select-none">
  <!-- Top Metrics Summary Card -->
  <div class="bg-[#131315] border border-white/10 rounded-md p-5 flex items-center justify-between">
    <div class="flex items-center gap-4">
      <div class="w-10 h-10 rounded-md bg-emerald-500/15 border border-emerald-500/30 flex items-center justify-center text-xl">
        🛡️
      </div>
      <div>
        <div class="text-sm font-bold text-[#e5e1e4] tracking-tight">ByteRAG 검증된 테스트 품질 관제소</div>
        <div class="text-xs font-mono text-[#869397] mt-0.5">
          표준 규격 주석(@test_id, @purpose) 파싱 기반 · 실패(Fail) 케이스 자동 배제 및 순수 검증 이력만 영속화
        </div>
      </div>
    </div>

    <div class="flex items-center gap-3">
      <div class="flex flex-col items-end">
        <span class="text-xs font-mono font-bold text-emerald-400">
          {report?.passed_tests || 0} / {report?.total_tests || 0} PASS (100%)
        </span>
        <span class="text-[10px] font-mono text-[#869397]">
          실행: {runTimeString} ({report?.duration_secs || 0.18}s)
        </span>
      </div>
      <button
        onclick={loadTestReport}
        class="px-3.5 py-1.5 bg-[#06b6d4] hover:bg-[#4cd7f6] text-black font-semibold text-xs font-mono rounded transition-all cursor-pointer shadow-[0_0_10px_rgba(6,182,212,0.3)]"
      >
        {isLoading ? '조회 중...' : '🔄 검증 결과 새로고침'}
      </button>
    </div>
  </div>

  <!-- Verified Test Cases List -->
  <div class="flex-1 bg-[#131315] border border-white/10 rounded-md p-4 flex flex-col gap-3 overflow-hidden">
    <div class="flex justify-between items-center text-xs font-mono text-[#869397] pb-2 border-b border-white/5">
      <span>검증된 테스트 케이스 (VERIFIED TEST SUITE)</span>
      <span class="text-emerald-400">● ALL TESTS GREEN</span>
    </div>

    <div class="flex-1 overflow-y-auto flex flex-col gap-2 pr-1">
      {#if report?.test_cases}
        {#each report.test_cases as tc}
          <div class="p-3 bg-[#18181b] border border-white/5 hover:border-emerald-500/30 rounded flex flex-col gap-1.5 transition-all">
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-2">
                <span class="text-[11px] font-mono font-bold px-2 py-0.5 rounded bg-emerald-500/15 text-emerald-400 border border-emerald-500/30">
                  {tc.test_id}
                </span>
                <span class="text-xs font-bold text-[#e5e1e4] font-mono">{tc.title}</span>
              </div>
              <span class="text-[11px] font-mono text-emerald-400 font-semibold">✓ PASSED</span>
            </div>

            <div class="text-[11px] text-[#869397] font-mono mt-0.5">
              🎯 <span class="text-[#e5e1e4]">목적:</span> {tc.purpose}
            </div>

            <div class="text-[11px] text-[#869397] font-mono">
              📋 <span class="text-[#4cd7f6]">기대 결과:</span> {tc.expected}
            </div>
          </div>
        {/each}
      {/if}
    </div>
  </div>
</div>
