// Simple reactive i18n store in standard Svelte 5 .svelte.js module (Default: English 'en')
let currentLang = $state(
  typeof localStorage !== 'undefined'
    ? localStorage.getItem('codeorbit_lang') || 'en'
    : 'en'
);

export function getLang() {
  return currentLang;
}

export function setLang(lang) {
  currentLang = lang;
  if (typeof localStorage !== 'undefined') {
    localStorage.setItem('codeorbit_lang', lang);
  }
}

const translations = {
  ko: {
    // Sidebar
    nav_overview: '⚡ AI 관제 & 대시보드',
    nav_docs: '🗄️ ByteRAG 문서 저장소',
    nav_tests: '🛡️ 테스트 품질 관제소',
    nav_mcp: '🔌 MCP 도구 카탈로그',
    nav_settings: '⚙️ 환경 설정 & 시작프로그램',
    engine_badge: 'BYTERAG 엔진',
    status_indexing: '인덱싱 중...',
    status_ready: 'MCP 준비 완료',

    // Workspace Selector
    workspace_label: '워크스페이스',
    add_workspace: '+ 워크스페이스 추가',
    btn_add: '추가',
    btn_cancel: '취소',
    btn_remove: '제거',
    btn_remove_title: '현재 워크스페이스 제거',
    prompt_copy: '🤖 AI 프롬프트 복사',
    prompt_copied: '✓ 클립보드 복사 완료!',
    reindex: '🔄 재인덱싱',
    export_brdb: '📦 단일 .brdb 변환',
    export_brdb_packing: '패킹 중...',
    export_brdb_done: '✓ .brdb 변환 완료',
    hide_tray: '🔽 트레이로 최소화',

    // Metrics Grid
    metric_files: '파싱 대상 파일',
    metric_files_desc: 'AST 분석 대상 소스코드',
    metric_nodes: '지식 심볼 (Nodes)',
    metric_nodes_desc: '클래스, 구조체, 함수, 인터페이스',
    metric_edges: '관계 엣지 (Edges)',
    metric_edges_desc: '상속, 구현, 임포트, 호출 관계',
    metric_engine: '저장소 엔진 모드',
    metric_engine_desc: 'ByteRAG 5T 임베디드 코어',

    // AI Audit & Plan Progress
    plan_title: 'AI 작업 계획 & 진행률',
    plan_subtitle: '실시간 마일스톤 트래커',
    plan_file: '계획 파일',
    plan_progress: '전체 진행률',
    plan_empty: '현재 등록된 작업 계획서가 없습니다.',
    audit_title: 'AI 실시간 활동 감사 스트림',
    audit_subtitle: 'MCP 도구 호출 및 영향도 로그',
    audit_no_logs: '기록된 AI 감사 로그가 없습니다.',

    // Blast Radius Viewer
    blast_title: 'AI 코드 변경 파급력 & GraphRAG 의존 체인 분석',
    blast_placeholder: '심볼명 (예: GraphStore, parse_file)...',
    blast_btn: '분석',
    blast_analyzing: '분석 중...',
    blast_symbol_label: '심볼',
    blast_nodes_found: '예상 파급 심볼',
    blast_depth: '탐색 깊이',

    // Docs Manager
    docs_tree_title: 'ByteRAG 문서 트리',
    docs_tree_sub: 'Zero Disk Mess (순수 DB 영속화)',
    docs_new_btn: '+ 새 문서',
    docs_search_ph: '문서 제목, 본문 키워드 검색...',
    docs_cat_all: '전체 문서',
    docs_cat_plan: '작업 계획서',
    docs_cat_spec: '요구 사양서',
    docs_cat_manual: '기능 매뉴얼',
    docs_cat_general: '일반 지식',
    docs_create_title_ph: '문서 제목 (예: 결제 모듈 사양서)',
    docs_create_btn: 'ByteRAG에 생성',
    docs_save_btn: '💾 ByteRAG DB에 저장',
    docs_saving: '저장 중...',
    docs_saved_msg: '✓ ByteRAG DB에 영속 저장되었습니다.',
    docs_delete_title: '문서 삭제 (ByteRAG DB에서 영구 제거)',
    docs_delete_confirm: '정말 문서를 ByteRAG에서 삭제하시겠습니까?',
    docs_no_selected: '선택된 문서가 없습니다. 좌측 트리에서 문서를 선택하거나 생성하세요.',
    docs_empty_search: '일치하는 문서가 없습니다.',

    // Test Verification
    test_title: 'ByteRAG 검증된 테스트 품질 관제소',
    test_sub: '표준 규격 주석(@test_id, @purpose) 파싱 기반 · 실패(Fail) 케이스 자동 배제 및 순수 검증 이력만 영속화',
    test_refresh: '🔄 검증 결과 새로고침',
    test_refreshing: '조회 중...',
    test_suite_header: '검증된 테스트 케이스',
    test_all_green: '● 전수 테스트 통과',
    test_search_ph: '테스트 ID, 기능명, 검증 목적, 기대 결과 키워드 실시간 검색...',
    test_purpose_label: '목적',
    test_expected_label: '기대 결과',
    test_empty_search: '일치하는 테스트 케이스가 없습니다.',

    // MCP Tab
    mcp_header_title: 'ByteRAG GraphRAG MCP 도구 카탈로그',
    mcp_header_sub: 'Cursor, Claude Desktop, Antigravity 등 모든 LLM 클라이언트와 통신 가능한 13개 고성능 도구',
    mcp_tool_count: '총 13개 도구 사용 가능',
    mcp_copy_cursor: '📋 Cursor MCP 설정 복사',
    mcp_copy_claude: '📋 Claude Desktop 설정 복사',
    mcp_copied: '✓ 설정이 클립보드에 복사되었습니다!',

    // Settings Tab
    settings_header: '시스템 및 시작프로그램 설정',
    settings_autostart_title: '윈도우 부팅 시 CodeOrbit 자동 실행 (Autostart)',
    settings_autostart_desc: '컴퓨터를 켤 때 시스템 트레이로 자동 실행되어 에디터(Cursor, Antigravity)가 시작될 때 지식 그래프가 즉시 준비됩니다.',
    settings_port_title: 'MCP 통신 포트 / 파이프',
    settings_port_desc: 'stdio 기반 글로벌 클라이언트 연동',
  },
  en: {
    // Sidebar
    nav_overview: '⚡ AI Control & Dashboard',
    nav_docs: '🗄️ ByteRAG Document Store',
    nav_tests: '🛡️ Test Quality Center',
    nav_mcp: '🔌 MCP Tool Catalog',
    nav_settings: '⚙️ Settings & Autostart',
    engine_badge: 'BYTERAG ENGINE',
    status_indexing: 'INDEXING...',
    status_ready: 'MCP READY',

    // Workspace Selector
    workspace_label: 'Workspace',
    add_workspace: '+ Add Workspace',
    btn_add: 'Add',
    btn_cancel: 'Cancel',
    btn_remove: 'Remove',
    btn_remove_title: 'Remove current workspace',
    prompt_copy: '🤖 Copy AI Prompt',
    prompt_copied: '✓ Copied to Clipboard!',
    reindex: '🔄 Reindex',
    export_brdb: '📦 Export .brdb Archive',
    export_brdb_packing: 'Packing...',
    export_brdb_done: '✓ Export Complete',
    hide_tray: '🔽 Minimize to Tray',

    // Metrics Grid
    metric_files: 'Indexed Files',
    metric_files_desc: 'Parsed source code files',
    metric_nodes: 'Knowledge Symbols (Nodes)',
    metric_nodes_desc: 'Classes, Structs, Functions, Interfaces',
    metric_edges: 'Dependency Edges',
    metric_edges_desc: 'Inherits, Implements, Imports, Calls',
    metric_engine: 'Storage Engine Mode',
    metric_engine_desc: 'ByteRAG 5T Embedded Core',

    // AI Audit & Plan Progress
    plan_title: 'AI Plan & Progress Tracker',
    plan_subtitle: 'Real-time milestone tracker',
    plan_file: 'Plan File',
    plan_progress: 'Total Progress',
    plan_empty: 'No active plan found. Create one in Document Store.',
    audit_title: 'AI Live Activity Audit Stream',
    audit_subtitle: 'MCP tool invocation and blast radius logs',
    audit_no_logs: 'No AI audit logs recorded yet.',

    // Blast Radius Viewer
    blast_title: 'AI Blast Radius & GraphRAG Dependency Viewer',
    blast_placeholder: 'Symbol name (e.g., GraphStore, parse_file)...',
    blast_btn: 'Analyze',
    blast_analyzing: 'Analyzing...',
    blast_symbol_label: 'Symbol',
    blast_nodes_found: 'Affected Symbols',
    blast_depth: 'Traversal Depth',

    // Docs Manager
    docs_tree_title: 'ByteRAG Document Tree',
    docs_tree_sub: 'Zero Disk Mess (Pure DB Persistence)',
    docs_new_btn: '+ New Doc',
    docs_search_ph: 'Search doc title, markdown body...',
    docs_cat_all: 'All Docs',
    docs_cat_plan: 'Plans',
    docs_cat_spec: 'Specs',
    docs_cat_manual: 'Manuals',
    docs_cat_general: 'General',
    docs_create_title_ph: 'Document title (e.g. Auth Architecture Spec)',
    docs_create_btn: 'Create in ByteRAG',
    docs_save_btn: '💾 Save to ByteRAG DB',
    docs_saving: 'Saving...',
    docs_saved_msg: '✓ Persisted to ByteRAG DB.',
    docs_delete_title: 'Delete document from ByteRAG DB',
    docs_delete_confirm: 'Are you sure you want to delete this document from ByteRAG?',
    docs_no_selected: 'No document selected. Choose or create a document from the left tree.',
    docs_empty_search: 'No matching documents found.',

    // Test Verification
    test_title: 'ByteRAG Verified Test Quality Center',
    test_sub: 'Standardized doc-comment (@test_id, @purpose) parser · Auto-discards failures, persists verified passing suites only',
    test_refresh: '🔄 Refresh Test Report',
    test_refreshing: 'Refreshing...',
    test_suite_header: 'Verified Test Suite',
    test_all_green: '● ALL TESTS GREEN',
    test_search_ph: 'Search test ID, function, purpose, expected result...',
    test_purpose_label: 'Purpose',
    test_expected_label: 'Expected Result',
    test_empty_search: 'No matching test cases found.',

    // MCP Tab
    mcp_header_title: 'ByteRAG GraphRAG MCP Tool Catalog',
    mcp_header_sub: '13 high-performance tools communicating with Cursor, Claude Desktop, Antigravity, and any LLM client',
    mcp_tool_count: '13 Tools Available',
    mcp_copy_cursor: '📋 Copy Cursor MCP Config',
    mcp_copy_claude: '📋 Copy Claude Desktop Config',
    mcp_copied: '✓ Config copied to clipboard!',

    // Settings Tab
    settings_header: 'SYSTEM & AUTOSTART CONFIGURATION',
    settings_autostart_title: 'Launch CodeOrbit on Windows Startup (Autostart)',
    settings_autostart_desc: 'Automatically launches in background tray upon boot so your AST graph is ready the moment you open your IDE.',
    settings_port_title: 'MCP Transport Protocol',
    settings_port_desc: 'stdio-based global client bridge',
  }
};

export function t(key) {
  const langObj = translations[currentLang] || translations.en;
  return langObj[key] || key;
}
