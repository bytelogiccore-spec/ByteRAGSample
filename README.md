# CodeOrbit - Embedded GraphRAG & AI Mission Control

> **Ultra-Fast AST Knowledge Graph & Mission Control Center for AI-Assisted Development**  
> Powered by **ByteRAG 5-Tier Embedded Storage Engine** (`.brdb`).

---

## 🌟 Overview

**CodeOrbit** is a high-performance, developer-centric desktop dashboard and MCP (Model Context Protocol) server designed to supercharge AI coding assistants like **Cursor, Antigravity, and Claude Desktop**.

Instead of treating LLM coding as a black box with messy disk files, CodeOrbit provides:
1. **Real-time AI Activity Audit Stream**: Monitor MCP tool calls and affected code symbols live.
2. **Zero-Disk-Mess Document Management**: Store project plans, architecture specs, and manuals directly inside the **ByteRAG 5T Database** with zero clutter in your git repo.
3. **Verified Test Quality Center**: Automatically parse standardized doc-comments (`@test_id`, `@purpose`) and keep a 100% clean record of verified passing suites (failing tests are auto-discarded from history).
4. **Instant Blast Radius & Multi-Hop Traversal**: Calculate dependency impact chains before modifying any code.
5. **Portable `.brdb` Single-File Archive**: Pack AST graphs, documentation, and verified test history into a single binary archive.

---

## 📐 Architecture & 5-Tier Storage

```
               [ 5 Supported Languages: Rust, C++, C#, TypeScript, Python ]
                                          │
                                          ▼
                            [ tree-sitter AST Parsers ]
                                          │
                                          ▼
┌─────────────────────────────────────────────────────────────────────────────────┐
│                          ByteRAG 5-Tier Embedded Core                           │
├─────────────────┬─────────────────┬──────────────────┬──────────────────────────┤
│ L0: WAL Buffer  │ L1: In-Mem WOS  │ L2: Arrow Scan   │ L3: CSR Graph Traversal  │
│ (Microsecond)   │ (Append-Only)   │ (Zero-Copy FTS)  │ (Multi-Hop BFS Explorer) │
└─────────────────┴─────────────────┴──────────────────┴──────────────────────────┘
                                          │
                    ┌─────────────────────┴─────────────────────┐
                    ▼                                           ▼
      [ MCP Server stdio Protocol ]              [ Tauri Desktop GUI & i18n ]
      (13 GraphRAG Tools for AI)                 (Svelte 5 + Tailwind Dashboard)
```

---

## 🚀 Key Features

### 1. 🤖 AI Live Audit & Plan Progress Tracker
- **Real-Time Feed**: Track `byterag_query_graph`, `byterag_blast_radius`, and `byterag_find_path` tool invocations in milliseconds.
- **Dynamic Milestone Progress**: Synchronizes active implementation milestones directly from ByteRAG DB.

### 2. 🗄️ Zero Disk Mess In-DB Document Store
- Write and edit Markdown plans, specs, and manuals directly in the app.
- Zero leftover `.md` files cluttering your project tree — persisted safely into ByteRAG's `docs` table.
- Categorized by **Plans (`📋`)**, **Specs (`📐`)**, **Manuals (`📖`)**, and **General (`📝`)** with real-time full-text search.

### 3. 🛡️ Verified Test Quality Center
- Standardized Rust test metadata:
  ```rust
  /// @test_id: TC-STORE-001
  /// @title: GraphStore Full Lifecycle & Blast Radius
  /// @purpose: Verify indexing, BFS query, and .brdb packaging
  /// @expected: 100% Passing test suite snapshot
  #[test]
  fn test_store_lifecycle_and_search() { ... }
  ```
- Displays verified passing tests in clean, human-readable Korean/English reports.

### 4. 🌐 Global Ready with Full i18n
- Seamless one-click toggle between **English (`EN`)** and **Korean (`KO`)**.
- Remembers user language preference via persistent storage.

---

## 🛠️ Quick Start

### Prerequisites
- [Rust](https://www.rust-lang.org/) (Edition 2021)
- [Node.js](https://nodejs.org/) & `npm`

### 1. Build Frontend & Desktop App
```bash
# Build Svelte 5 static frontend
cd crates/codeorbit/frontend
npm install
npm run build

# Build Tauri desktop executable
cd ../../..
cargo build -p codeorbit
```

### 2. Launch CodeOrbit
```powershell
& ".\target\debug\codeorbit.exe"
```

### 3. Register with Cursor / Claude Desktop (MCP Config)
Add the following to your `mcpServers` configuration (`mcp.json` or `claude_desktop_config.json`):

```json
{
  "mcpServers": {
    "codeorbit": {
      "command": "d:\\ByteLogicCore\\ByteRAGSample\\target\\debug\\codeorbit.exe",
      "args": []
    }
  }
}
```

---

## 📦 License
Licensed under the Apache-2.0 License.
