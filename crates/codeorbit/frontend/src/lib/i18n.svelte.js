// Simple reactive i18n store in standard Svelte 5 .svelte.js module
let currentLang = $state(
  typeof localStorage !== 'undefined'
    ? localStorage.getItem('codeorbit_lang') || 'ko'
    : 'ko'
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
    nav_overview: '⚡ AI 관제 & 대시보드',
    nav_docs: '🗄️ ByteRAG 문서 저장소',
    nav_tests: '🛡️ 테스트 품질 관제소',
    nav_mcp: '🔌 MCP 도구 카탈로그',
    nav_settings: '⚙️ 환경 설정 & 시작프로그램',
    engine_badge: 'BYTERAG 엔진',
    status_indexing: '인덱싱 중...',
    status_ready: 'MCP 준비 완료',

    workspace_label: '워크스페이스',
    add_workspace: '+ 워크스페이스 추가',
    prompt_copy: '🤖 AI 프롬프트 복사',
    prompt_copied: '✓ 클립보드 복사 완료!',
    reindex: '🔄 재인덱싱',
    export_brdb: '📦 단일 .brdb 변환',
    hide_tray: '🔽 트레이로 최소화',

    metric_files: '파싱 대상 파일',
    metric_files_desc: 'AST 분석 대상 소스코드',
    metric_nodes: '지식 심볼 (Nodes)',
    metric_nodes_desc: '클래스, 구조체, 함수, 인터페이스',
    metric_edges: '관계 엣지 (Edges)',
    metric_edges_desc: '상속, 구현, 임포트, 호출 관계',
    metric_engine: '저장소 엔진 모드',
    metric_engine_desc: 'ByteRAG 5T 임베디드 코어',

    plan_title: 'AI 작업 계획 & 진행률',
    plan_subtitle: '실시간 마일스톤 트래커',
    plan_file: '계획 파일',
    plan_progress: '전체 진행률',
    audit_title: 'AI 실시간 활동 감사 스트림',
    audit_subtitle: 'MCP 도구 호출 및 영향도 로그',
    audit_no_logs: '감사 로그가 없습니다.',

    blast_title: 'GraphRAG 영향도 & 의존성 체인 뷰어',
    blast_placeholder: '심볼 검색 (예: GraphStore, parse_file)...',
    blast_btn: '영향도 분석',
    blast_nodes_found: '영향 받는 노드',
    blast_depth: '탐색 깊이',

    docs_tree_title: 'ByteRAG 문서 트리',
    docs_tree_sub: 'Zero Disk Mess (순수 DB 영속화)',
    docs_new_btn: '+ 새 문서',
    docs_search_ph: '문서 제목, 본문 키워드 검색...',
    docs_cat_all: '전체 문서',
    docs_cat_plan: '작업 계획서',
    docs_cat_spec: '요구 사양서',
    docs_cat_manual: '기능 매뉴얼',
    docs_cat_general: '일반 지식',
    docs_save_btn: '💾 ByteRAG DB에 저장',
    docs_saving: '저장 중...',
    docs_saved_msg: '✓ ByteRAG DB에 영속 저장되었습니다.',
    docs_no_selected: '선택된 문서가 없습니다. 좌측 트리에서 문서를 선택하거나 생성하세요.',

    test_title: 'ByteRAG 검증된 테스트 품질 관제소',
    test_sub: '표준 규격 주석(@test_id, @purpose) 파싱 기반 · 실패(Fail) 케이스 자동 배제 및 순수 검증 이력만 영속화',
    test_refresh: '🔄 검증 결과 새로고침',
    test_refreshing: '조회 중...',
    test_suite_header: '검증된 테스트 케이스',
    test_all_green: '● 전수 테스트 통과',
    test_search_ph: '테스트 ID, 기능명, 검증 목적, 기대 결과 키워드 실시간 검색...',
    test_purpose_label: '목적',
    test_expected_label: '기대 결과',

    mcp_header_title: 'ByteRAG GraphRAG MCP 도구 카탈로그',
    mcp_header_sub: 'Cursor, Claude Desktop, Antigravity 등 모든 LLM 클라이언트와 통신 가능한 13개 고성능 도구',
    mcp_tool_count: '총 13개 도구 사용 가능',

    settings_title: '환경 설정 & 시스템 연동',
    settings_sub: '시스템 시작 시 자동 실행 및 백그라운드 상주 설정',
    settings_autostart_title: 'Windows 시작 시 CodeOrbit 자동 실행',
    settings_autostart_desc: '부팅 시 트레이 아이콘으로 백그라운드 시작',
    settings_port_title: 'MCP 통신 포트 / 파이프',
    settings_port_desc: 'stdio 기반 글로벌 클라이언트 연동',
  },
  en: {
    nav_overview: '⚡ AI Control & Dashboard',
    nav_docs: '🗄️ ByteRAG Document Store',
    nav_tests: '🛡️ Test Quality Center',
    nav_mcp: '🔌 MCP Tool Catalog',
    nav_settings: '⚙️ Settings & Autostart',
    engine_badge: 'BYTERAG ENGINE',
    status_indexing: 'INDEXING...',
    status_ready: 'MCP READY',

    workspace_label: 'Workspace',
    add_workspace: '+ Add Workspace',
    prompt_copy: '🤖 Copy AI Prompt',
    prompt_copied: '✓ Copied to Clipboard!',
    reindex: '🔄 Reindex',
    export_brdb: '📦 Export .brdb Archive',
    hide_tray: '🔽 Minimize to Tray',

    metric_files: 'Indexed Files',
    metric_files_desc: 'Parsed source code files',
    metric_nodes: 'Knowledge Symbols (Nodes)',
    metric_nodes_desc: 'Classes, Structs, Functions, Interfaces',
    metric_edges: 'Dependency Edges',
    metric_edges_desc: 'Inherits, Implements, Imports, Calls',
    metric_engine: 'Storage Engine Mode',
    metric_engine_desc: 'ByteRAG 5T Embedded Core',

    plan_title: 'AI Plan & Progress Tracker',
    plan_subtitle: 'Real-time milestone tracker',
    plan_file: 'Plan File',
    plan_progress: 'Total Progress',
    audit_title: 'AI Live Activity Audit Stream',
    audit_subtitle: 'MCP tool invocation and blast radius logs',
    audit_no_logs: 'No audit logs recorded yet.',

    blast_title: 'GraphRAG Blast Radius & Dependency Viewer',
    blast_placeholder: 'Search symbol (e.g., GraphStore, parse_file)...',
    blast_btn: 'Analyze Blast Radius',
    blast_nodes_found: 'Affected Nodes',
    blast_depth: 'Traversal Depth',

    docs_tree_title: 'ByteRAG Document Tree',
    docs_tree_sub: 'Zero Disk Mess (Pure DB Persistence)',
    docs_new_btn: '+ New Doc',
    docs_search_ph: 'Search doc title, markdown body...',
    docs_cat_all: 'All Docs',
    docs_cat_plan: 'Plans',
    docs_cat_spec: 'Specs',
    docs_cat_manual: 'Manuals',
    docs_cat_general: 'General',
    docs_save_btn: '💾 Save to ByteRAG DB',
    docs_saving: 'Saving...',
    docs_saved_msg: '✓ Persisted to ByteRAG DB.',
    docs_no_selected: 'No document selected. Choose or create a document from the left tree.',

    test_title: 'ByteRAG Verified Test Quality Center',
    test_sub: 'Standardized doc-comment (@test_id, @purpose) parser · Auto-discards failures, persists verified passing suites only',
    test_refresh: '🔄 Refresh Test Report',
    test_refreshing: 'Refreshing...',
    test_suite_header: 'Verified Test Suite',
    test_all_green: '● ALL TESTS GREEN',
    test_search_ph: 'Search test ID, function, purpose, expected result...',
    test_purpose_label: 'Purpose',
    test_expected_label: 'Expected Result',

    mcp_header_title: 'ByteRAG GraphRAG MCP Tool Catalog',
    mcp_header_sub: '13 high-performance tools communicating with Cursor, Claude Desktop, Antigravity, and any LLM client',
    mcp_tool_count: '13 Tools Available',

    settings_title: 'Settings & System Integration',
    settings_sub: 'Configure system startup and background tray behavior',
    settings_autostart_title: 'Launch CodeOrbit on Windows Startup',
    settings_autostart_desc: 'Start automatically in background tray on boot',
    settings_port_title: 'MCP Transport Protocol',
    settings_port_desc: 'stdio-based global client bridge',
  }
};

export function t(key) {
  const langObj = translations[currentLang] || translations.ko;
  return langObj[key] || key;
}
