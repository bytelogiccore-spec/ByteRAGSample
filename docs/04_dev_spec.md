# 🛠️ 개발자 사양서 및 기여 가이드 (Developer Specifications)

## 1. 디렉토리 구조 및 워크스페이스

```text
ByteRAGSample/
├── Cargo.toml                     # 통합 워크스페이스 정의
├── docs/                          # 시스템 및 사용자 문서
│   ├── 01_system_overview.md      # 시스템 아키텍처 개요
│   ├── 02_ui_manual.md            # UI 사용자 매뉴얼
│   ├── 03_mcp_tools_reference.md  # MCP 도구 레퍼런스
│   └── 04_dev_spec.md             # 개발자 사양서 (본 문서)
├── crates/
│   ├── byterag-codegraph/         # Core AST & GraphRAG 라이브러리 & Stdio MCP
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs             # 공용 라이브러리 export
│   │       ├── main.rs            # Stdio MCP Server 엔트리포인트
│   │       ├── parser.rs          # 다국어 AST 파서
│   │       ├── store.rs           # CsrGraph + ByteRAG DB 연동 스토어
│   │       └── types.rs           # Node / Edge / AST 타입 정의
│   └── codeorbit/                 # Tauri 2 Desktop GUI & Tray & Autostart
│       ├── Cargo.toml
│       ├── tauri.conf.json        # Tauri 2 앱 설정 및 권한 매핑
│       ├── capabilities/          # Tauri 2 보안 권한 매니페스트
│       ├── src/
│       │   └── main.rs            # Tauri 백엔드 커맨드 및 트레이 핸들러
│       └── ui/
│           └── index.html         # Cyber-Kinetic 개발자 대시보드
```

---

## 2. 개발 및 빌드 명령어

### 1) 데스크톱 앱 개발 모드 실행
```powershell
cd d:\ByteLogicCore\ByteRAGSample\crates\codeorbit
cargo tauri dev
```

### 2) 배포용 단독 실행 파일 빌드
```powershell
cd d:\ByteLogicCore\ByteRAGSample\crates\codeorbit
cargo tauri build
```
- 빌드 결과물: `target/release/codeorbit.exe`

### 3) MCP 서버 단독 빌드 및 테스트
```powershell
cargo build -p byterag-codegraph
```
- 결과물: `target/debug/byterag_sample.exe`

---

## 3. 핵심 기술 스택

- **Backend**: Rust 2021, `byterag-core 0.3.0` (Pure Rust 5-Tier KV & Graph DB), `tokio`, `serde_json`
- **Frontend / GUI**: Tauri 2.x, Webview2, HTML5 / CSS3 / Vanilla JS (No heavy node_modules build step required)
- **Design System**: Stitch AI Generated `Cyber-Kinetic Developer Core`
- **Protocols**: MCP (Model Context Protocol) JSON-RPC 2.0 over Stdio, Tauri IPC IPC-Bridge
