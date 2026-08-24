<script>
  import './app.css';
  import Sidebar from './components/Sidebar.svelte';
  import WorkspaceSelector from './components/WorkspaceSelector.svelte';
  import MetricsGrid from './components/MetricsGrid.svelte';
  import PlanProgressCard from './components/PlanProgressCard.svelte';
  import AiAuditStream from './components/AiAuditStream.svelte';
  import BlastRadiusViewer from './components/BlastRadiusViewer.svelte';
  import DocsManagerTab from './components/DocsManagerTab.svelte';
  import TestVerificationTab from './components/TestVerificationTab.svelte';
  import McpToolsTab from './components/McpToolsTab.svelte';
  import SettingsTab from './components/SettingsTab.svelte';
  import { invokeCommand } from './lib/tauri.js';

  let activeTab = $state('tab-overview');
  let status = $state({
    files: 0,
    nodes: 0,
    edges: 0,
    indexing: false,
    last_indexed_at: null,
    target_dir: ''
  });
  let workspaces = $state([]);
  let activePath = $state('');
  let planStatus = $state({});
  let auditLogs = $state([]);

  async function loadWorkspaces() {
    try {
      const res = await invokeCommand('get_workspaces');
      if (res) {
        workspaces = res.workspaces || [];
        activePath = res.active_path || (workspaces[0] ? workspaces[0].path : '');
      }
    } catch (e) {
      console.warn('loadWorkspaces error:', e);
    }
  }

  async function syncStatus() {
    try {
      const [s, plan, logs] = await Promise.all([
        invokeCommand('get_index_status'),
        invokeCommand('get_project_plan_status'),
        invokeCommand('get_ai_audit_logs')
      ]);
      if (s) status = s;
      if (plan) planStatus = plan;
      if (logs) auditLogs = logs;
    } catch (e) {
      console.warn('syncStatus error:', e);
    }
  }

  async function handleSelectWorkspace(path) {
    activePath = path;
    try {
      await invokeCommand('trigger_reindex', { targetDir: path, target_dir: path });
      await syncStatus();
    } catch (e) {
      alert('Failed to switch workspace: ' + e);
    }
  }

  async function handleAddWorkspace(path) {
    try {
      const res = await invokeCommand('add_workspace', { path });
      if (res) {
        workspaces = res.workspaces || [];
        activePath = res.active_path || path;
      }
      await invokeCommand('trigger_reindex', { targetDir: path, target_dir: path });
      await syncStatus();
    } catch (e) {
      alert('Failed to add workspace: ' + e);
    }
  }

  async function handleRemoveWorkspace(path) {
    try {
      const res = await invokeCommand('remove_workspace', { path });
      if (res) {
        workspaces = res.workspaces || [];
        activePath = res.active_path;
        if (activePath) {
          await invokeCommand('trigger_reindex', { targetDir: activePath, target_dir: activePath });
          await syncStatus();
        }
      }
    } catch (e) {
      alert('Failed to remove workspace: ' + e);
    }
  }

  async function handleReindex() {
    try {
      await invokeCommand('trigger_reindex', { targetDir: activePath, target_dir: activePath });
      await syncStatus();
    } catch (e) {
      alert('Reindex request failed: ' + e);
    }
  }

  async function handleHideToTray() {
    try {
      await invokeCommand('hide_to_tray');
    } catch (e) {}
  }

  $effect(() => {
    loadWorkspaces();
    syncStatus();
    const timer = setInterval(syncStatus, 2500);
    return () => clearInterval(timer);
  });
</script>

<div class="flex h-screen w-screen bg-[#09090b] text-[#e5e1e4] overflow-hidden select-none">
  <!-- Left Responsive Sidebar -->
  <Sidebar bind:activeTab {status} />

  <!-- Main Canvas -->
  <div class="flex-1 flex flex-col overflow-hidden min-w-0">
    <!-- Top Workspace Selector Bar -->
    <WorkspaceSelector
      {workspaces}
      {activePath}
      onSelect={handleSelectWorkspace}
      onAdd={handleAddWorkspace}
      onRemove={handleRemoveWorkspace}
      onReindex={handleReindex}
      onHideToTray={handleHideToTray}
    />

    <!-- Main Dynamic Tab View with Responsive Stack Layout -->
    <div class="flex-1 p-3.5 lg:p-6 overflow-y-auto flex flex-col gap-4 custom-scrollbar">
      {#if activeTab === 'tab-overview'}
        <!-- 4 Metrics Cards (Dynamic 1-4 Grid) -->
        <MetricsGrid {status} />

        <!-- Responsive Split View: 1 Column on Mobile/Tablet, 12 Columns on Desktop -->
        <div class="grid grid-cols-1 lg:grid-cols-12 gap-4">
          <div class="lg:col-span-7">
            <PlanProgressCard {planStatus} />
          </div>
          <div class="lg:col-span-5">
            <AiAuditStream {auditLogs} />
          </div>
        </div>

        <!-- Blast Radius & GraphRAG Traversal Viewer -->
        <BlastRadiusViewer />
      {:else if activeTab === 'tab-docs'}
        <DocsManagerTab />
      {:else if activeTab === 'tab-tests'}
        <TestVerificationTab />
      {:else if activeTab === 'tab-mcp'}
        <McpToolsTab />
      {:else if activeTab === 'tab-settings'}
        <SettingsTab />
      {/if}
    </div>
  </div>
</div>
