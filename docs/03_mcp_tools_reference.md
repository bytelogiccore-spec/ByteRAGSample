# 🔌 CodeOrbit MCP 도구 레퍼런스 (MCP Tools Reference)

CodeOrbit은 **Model Context Protocol (JSON-RPC Stdio)**을 통해 AI 어시스턴트에게 총 13종의 지식 그래프 분석 도구를 제공합니다.

---

## 1. 지원 도구 목록 및 상세 사양

### 1) 상태 및 관리 도구

| 도구명 | 파라미터 | 설명 |
| :--- | :--- | :--- |
| **`byterag_index_status`** | `{}` | 현재 인덱싱 상태, 노드 수, 엣지 수, 파일 수, 타겟 경로 조회 |
| **`byterag_reindex`** | `target_dir` (선택) | 증분 mtime 기반 즉시 재인덱싱 수행 및 타겟 경로 변경 |
| **`byterag_export_brdb`** | `path`, `format_version` | 현재 구축된 인메모리/디스크 그래프를 `.brdb` 단일 아카이브로 덤프 |
| **`byterag_import_brdb`** | `path` (필수) | `.brdb` 파일을 읽어 라이브 스토어 교체 |

---

### 2) 심볼 검색 및 조회 도구

| 도구명 | 파라미터 | 설명 |
| :--- | :--- | :--- |
| **`byterag_search_symbols`** | `query` (필수), `limit` | 대소문자 무시 키워드 기반 심볼 검색 및 파일 위치 반환 |
| **`byterag_get_symbol`** | `node_id` 또는 `query` | 단일 심볼의 정의 라인, 파일, 소스 시그니처 정밀 조회 |
| **`byterag_list_by_type`** | `node_type` (필수), `language`, `path_prefix`, `limit` | 타입별 심볼 일괄 나열 (`struct`, `function`, `trait`, `file` 등) |
| **`byterag_read_snippet`** | `node_id` (필수), `before`, `after` | 심볼 주변 소스코드 라인을 읽어 컨텍스트 스니펫 제공 |

---

### 3) 그래프 탐색 및 구조 분석 도구

| 도구명 | 파라미터 | 설명 |
| :--- | :--- | :--- |
| **`byterag_get_neighbors`** | `node_id` (필수), `edge_types` | 특정 심볼의 1-hop 직속 인접 노드 및 연결 관계 간선 반환 |
| **`byterag_query_graph`** | `node_id` 또는 `node_ids`, `max_depth`, `edge_types` | **CsrGraph BFS 기반 양방향 서브그래프** 다중 홉 탐색 및 허브 요약 |
| **`byterag_blast_radius`** | `node_id` (필수), `max_depth` | 특정 심볼/함수 변경 시 **영향받는 상위 호출자/모듈 도달 범위** 분석 |
| **`byterag_find_path`** | `from` (필수), `to` (필수), `max_depth` | 두 심볼 간의 **최단 의존/호출 경로** 계산 |
| **`byterag_detect_cycles`** | `max_cycles` | imports, extends, implements, calls 간의 **순환 참조(Circular Dependency) 탐지** |

---

## 2. MCP 설정 방법 (IDE 연동)

`~/.gemini/antigravity/mcp_config.json` 또는 Cursor MCP 설정:

```json
{
  "mcpServers": {
    "codeorbit": {
      "command": "d:/ByteLogicCore/ByteRAGSample/target/debug/byterag_sample.exe",
      "args": [],
      "env": {
        "BYTERAG_TARGET_DIR": "d:/ByteLogicCore/ByteRAGSample"
      }
    }
  }
}
```
