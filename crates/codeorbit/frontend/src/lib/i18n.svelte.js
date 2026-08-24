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

    // MCP Tools Descriptions (Korean)
    tool_desc_query_graph: 'BFS 너비 우선 탐색을 통해 1~3단계 연결된 노드 및 관계 엣지 조회',
    tool_desc_search_symbols: 'Apache Arrow 제로카피 메모리 기반 초고속 심볼 접두사 및 부분 일치 검색',
    tool_desc_blast_radius: '심볼 수정 전 역방향 의존 체인 및 다중 홉(Multi-Hop) 파급 영향 범위 실시간 계산',
    tool_desc_find_path: '지식 그래프 내 임의의 두 심볼 간 최단 의존 경로 및 호출 체인 탐색',
    tool_desc_detect_cycles: '임포트, 상속, 인터페이스 구현, 함수 호출 간의 순환 참조(Circular Dependency) 탐지',
    tool_desc_reindex: '수정되거나 변경된 워크스페이스 소스 파일에 대해 즉시 증분 인덱싱 실행',
    tool_desc_export_brdb: '전체 AST 그래프 + 문서 저장소 + 검증 테스트 스위트를 단일 포터블 .brdb 파일로 압축 패킹',
    tool_desc_import_brdb: '포터블 .brdb 단일 아카이브 파일로부터 전체 지식 그래프와 문서를 즉시 복원 및 마운트',
    tool_desc_index_status: '현재 파싱된 파일 수, 심볼 노드, 변경 플래그 및 WAL/WOS 메모리 상태 검사',
    tool_desc_get_symbol: '특정 심볼 노드의 전체 메타데이터, 정의 위치 및 소스 코드 파일 경로 조회',
    tool_desc_get_neighbors: '특정 심볼과 직접 1단계(1-Hop) 연결된 진입/진출 이웃 심볼 노드 목록 조회',
    tool_desc_list_by_type: 'AST 타입별(구조체, 클래스, 함수, 인터페이스, 트레이트 등) 필터링 심볼 목록 조회',
    tool_desc_read_snippet: '특정 AST 심볼 노드가 선언된 정확한 소스 코드 원본 라인 및 스니펫 읽기',

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

    // MCP Tools Descriptions (English)
    tool_desc_query_graph: 'Query nodes and edges via BFS traversal (Depth 1-3)',
    tool_desc_search_symbols: 'Ultra-fast symbol prefix & substring search using Apache Arrow zero-copy memory',
    tool_desc_blast_radius: 'Calculate multi-hop impact radius and reverse dependencies before modifying symbols',
    tool_desc_find_path: 'Find shortest dependency path between any two symbols in the knowledge graph',
    tool_desc_detect_cycles: 'Detect circular dependencies across imports, extends, implements, and calls',
    tool_desc_reindex: 'Trigger incremental indexing on modified workspace source files',
    tool_desc_export_brdb: 'Pack full AST graph + docs + test suites into a single portable .brdb archive',
    tool_desc_import_brdb: 'Restore full AST graph and docs instantly from a portable .brdb file',
    tool_desc_index_status: 'Inspect current file counts, symbol nodes, dirty flags, and WAL memory state',
    tool_desc_get_symbol: 'Retrieve full metadata and location of a specific symbol node',
    tool_desc_get_neighbors: 'Get direct 1-hop inbound and outbound connected neighbor symbols',
    tool_desc_list_by_type: 'List symbols filtered by AST type (Struct, Class, Function, Interface)',
    tool_desc_read_snippet: 'Read exact code snippet lines for a specific AST node',

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
