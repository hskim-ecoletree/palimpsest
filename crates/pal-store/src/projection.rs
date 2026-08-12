//! 2층 — 질의 투영.
//!
//! # 왜 자체 인덱스인가 (stack §2.3 · F05 §2)
//!
//! 필요한 연산은 넷뿐이다 — 키 조회 / 인접 순회 / 역방향 색인 / **예산 절단이 있는**
//! 제한 깊이 탐색. 결정적인 이유는 넷째다: 질의 도중 잘라내고 **얼마나 무슨 이유로
//! 잘랐는지 응답에 실어야** 하는데, `LIMIT` 은 *"한도에 걸린 지점의 사유별 분해"* 를
//! 표현하지 못한다. 조용한 절단 금지는 이 제품의 정체성이다.
//!
//! # 이것은 캐시다 — 그리고 그 주장이 검사된다
//!
//! 2층에만 있는 상태는 존재하지 않는다. 통째로 지우고 1층에서 재구축하면 같은 값이
//! 나와야 하고, 그것이 S2 합격선 ⑤ 다(`corpus/criteria.toml` `[s2.pass].rebuild_equivalence`).
//! 다르면 2층은 캐시가 아니라 원본이고, 그 순간 *"자체 구현의 최대 비용이 0"* 이라는
//! 주장이 무너진다.
//!
//! # S2 가 세우는 것은 두 자리뿐이다
//!
//! F05 의 레이아웃은 열 몇 개를 적었다. 엣지·결박 색인·질의 로그는 그것을 채울 기능
//! (F07·F09~F12·F17)이 아직 없으므로 **자리도 만들지 않는다** — 빈 자리를 미리 만들면
//! 그것이 곧 "있는데 비어 있다"로 읽힌다.

use std::path::Path;

use pal_core::{SymbolId, SymbolNode};
use redb::{
    Database, MultimapTableDefinition, ReadableDatabase, ReadableTableMetadata, TableDefinition,
};

/// `symbol_id` → 심볼. 키가 32바이트 요약이다.
const SYMBOL: TableDefinition<&[u8], Vec<u8>> = TableDefinition::new("symbol");

/// 이름 → `symbol_id` 들. **사람은 해시로 묻지 않는다.**
const BY_NAME: MultimapTableDefinition<&str, &[u8]> = MultimapTableDefinition::new("by_name");

#[derive(Debug, thiserror::Error)]
pub enum ProjectionError {
    #[error("2층을 열지 못했다: {0}")]
    Open(String),
    #[error("2층 트랜잭션이 실패했다: {0}")]
    Transaction(String),
    #[error("2층 값을 풀지 못했다: {0}")]
    Decode(String),
}

/// 질의 투영. **`index.redb` 다** — 의도 저장소와 파일이 갈려 있다(R-21).
pub struct Projection {
    db: Database,
}

impl Projection {
    /// # Errors
    /// 파일을 열지 못하면.
    pub fn open(path: &Path) -> Result<Self, ProjectionError> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)
                .map_err(|e| ProjectionError::Open(format!("{}: {e}", dir.display())))?;
        }
        let db = Database::create(path)
            .map_err(|e| ProjectionError::Open(format!("{}: {e}", path.display())))?;
        Ok(Self { db })
    }

    /// **통째로 다시 만든다.** 증분이 아니다 — 2층은 캐시이고, 부분 갱신은 F05 의 것이다.
    ///
    /// # Errors
    /// 쓰기가 실패하면.
    pub fn rebuild(&self, symbols: &[SymbolNode]) -> Result<usize, ProjectionError> {
        let write = self.db.begin_write().map_err(|e| ProjectionError::Transaction(e.to_string()))?;

        // **테이블을 통째로 지우고 다시 만든다.** 남은 것이 있으면 "재구축했는데 옛 값이
        // 나오는" 상태가 되고, 그 순간 합격선 ⑤ 가 재는 등가성이 성립하지 않는다.
        write.delete_table(SYMBOL).map_err(|e| ProjectionError::Transaction(e.to_string()))?;
        write
            .delete_multimap_table(BY_NAME)
            .map_err(|e| ProjectionError::Transaction(e.to_string()))?;
        {
            let mut by_id =
                write.open_table(SYMBOL).map_err(|e| ProjectionError::Transaction(e.to_string()))?;
            let mut by_name = write
                .open_multimap_table(BY_NAME)
                .map_err(|e| ProjectionError::Transaction(e.to_string()))?;

            for s in symbols {
                let raw = postcard::to_allocvec(s)
                    .map_err(|e| ProjectionError::Decode(e.to_string()))?;
                by_id
                    .insert(s.id.as_bytes().as_slice(), raw)
                    .map_err(|e| ProjectionError::Transaction(e.to_string()))?;
                by_name
                    .insert(s.name.as_str(), s.id.as_bytes().as_slice())
                    .map_err(|e| ProjectionError::Transaction(e.to_string()))?;
            }
        }
        write.commit().map_err(|e| ProjectionError::Transaction(e.to_string()))?;
        Ok(symbols.len())
    }

    /// 좌표로 심볼 하나.
    ///
    /// # Errors
    /// 읽기가 실패하거나 값을 풀지 못하면.
    pub fn symbol(&self, id: SymbolId) -> Result<Option<SymbolNode>, ProjectionError> {
        let read = self.db.begin_read().map_err(|e| ProjectionError::Transaction(e.to_string()))?;
        // 아직 아무것도 안 들어간 2층이면 테이블 자체가 없다.
        let Ok(by_id) = read.open_table(SYMBOL) else {
            return Ok(None);
        };
        let got = by_id
            .get(id.as_bytes().as_slice())
            .map_err(|e| ProjectionError::Transaction(e.to_string()))?;
        match got {
            None => Ok(None),
            Some(v) => Ok(Some(
                postcard::from_bytes(&v.value()).map_err(|e| ProjectionError::Decode(e.to_string()))?,
            )),
        }
    }

    /// 이름으로 후보를 찾는다. **여럿일 수 있고, 그것이 정상이다.**
    ///
    /// # Errors
    /// 읽기가 실패하면.
    pub fn resolve_name(&self, name: &str) -> Result<Vec<SymbolNode>, ProjectionError> {
        let read = self.db.begin_read().map_err(|e| ProjectionError::Transaction(e.to_string()))?;
        let Ok(by_name) = read.open_multimap_table(BY_NAME) else {
            return Ok(Vec::new());
        };
        let Ok(by_id) = read.open_table(SYMBOL) else {
            return Ok(Vec::new());
        };

        let mut out: Vec<SymbolNode> = Vec::new();
        let ids = by_name.get(name).map_err(|e| ProjectionError::Transaction(e.to_string()))?;
        for id in ids {
            let id = id.map_err(|e| ProjectionError::Transaction(e.to_string()))?;
            if let Some(v) = by_id
                .get(id.value())
                .map_err(|e| ProjectionError::Transaction(e.to_string()))?
            {
                out.push(
                    postcard::from_bytes(&v.value())
                        .map_err(|e| ProjectionError::Decode(e.to_string()))?,
                );
            }
        }
        // **결정적 순서** — 같은 질의가 같은 순서를 내야 산출을 비교할 수 있다.
        out.sort_by(|a, b| a.path.cmp(&b.path).then_with(|| a.span.line_start.cmp(&b.span.line_start)));
        Ok(out)
    }

    /// 2층에 들어 있는 심볼 수. 봉투의 `projection.symbols_indexed` 가 이것이다.
    ///
    /// # Errors
    /// 읽기가 실패하면.
    pub fn count(&self) -> Result<usize, ProjectionError> {
        let read = self.db.begin_read().map_err(|e| ProjectionError::Transaction(e.to_string()))?;
        let Ok(by_id) = read.open_table(SYMBOL) else {
            return Ok(0);
        };
        let n: u64 = by_id.len().map_err(|e: redb::StorageError| ProjectionError::Transaction(e.to_string()))?;
        Ok(usize::try_from(n).unwrap_or(usize::MAX))
    }
}
