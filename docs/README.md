# 📚 CodeOrbit 종합 문서 센터 (Documentation Index)

CodeOrbit 프로젝트의 아키텍처, UI 사용법, MCP 도구 명세, 개발 사양서를 모아둔 공식 문서 모음입니다.

---

## 📑 목차

1. [01. 시스템 개요 (System Overview)](01_system_overview.md)
   - CodeOrbit이 무엇인지, 왜 필요한지, 기존 단순 RAG와의 차이점
   - 전체 시스템 아키텍처 다이어그램 (Desktop/Tray - Tauri - Rust Core - MCP - AI IDE)

2. [02. UI 사용자 매뉴얼 (UI Manual)](02_ui_manual.md)
   - 화면 레이아웃 및 4대 실시간 메트릭 카드 설명
   - 타겟 프로젝트 디렉터리 변경 방법
   - 실시간 심볼 검색 및 스니펫 조회 방법
   - 시스템 트레이(Tray) 상주 및 윈도우 시작프로그램(Autostart) 설정 방법

3. [03. MCP 도구 레퍼런스 (MCP Tools Reference)](03_mcp_tools_reference.md)
   - 13종 JSON-RPC Stdio MCP 도구 상세 명세 및 파라미터
   - `byterag_query_graph`, `byterag_blast_radius`, `byterag_find_path`, `byterag_detect_cycles` 등
   - Antigravity / Cursor IDE 연동 설정 방법

4. [04. 개발자 사양서 (Developer Specifications)](04_dev_spec.md)
   - 전체 워크스페이스 디렉터리 구조 및 크레이트 역할
   - 빌드 및 실행 명령어 (`cargo tauri dev`, `cargo tauri build`)
   - 핵심 기술 스택 및 데이터 흐름
