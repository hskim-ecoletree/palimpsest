//! 의도 저장소 — **결박의 원본.**
//!
//! # 이 파일에 지우는 함수가 없다. 그것이 대응이다 ([R-21])
//!
//! | | 1층 | 2층 | **의도 저장소** |
//! |---|---|---|---|
//! | 원본은 어디 | git | 1층 + 의도 저장소 | **자기 자신** |
//! | 지워도 되나 | 예 (재파싱) | 예 (재구축) | **아니오 — 유실** |
//!
//! 2층에 의도가 살면 *"지우고 재구축"* 이 사람의 노동을 지우는 명령이 되고,
//! **재구축 등가성 검사는 그 상태에서도 통과하므로 검사가 유실을 정상으로 승인한다.**
//!
//! 파일도 갈라져 있다 — `intent.redb` 와 `index.redb`. **파일이 갈린 것 자체가 대응이다**
//! (stack §2.4). 같은 파일에 두면 *"2층을 지운다"* 가 실수 하나로 의도를 지운다.

use std::path::Path;

use pal_core::{Binding, BindingId, SymbolId};
use redb::{
    Database, MultimapTableDefinition, ReadableDatabase, ReadableTable, ReadableTableMetadata,
    TableDefinition,
};

/// 결박 실체.
const BINDING: TableDefinition<&str, Vec<u8>> = TableDefinition::new("binding");

/// 대상 심볼 → 결박들. **역방향 색인** — `touch` 가 이것을 읽는다.
const BOUND_BY: MultimapTableDefinition<&[u8], &str> = MultimapTableDefinition::new("bound_by");

#[derive(Debug, thiserror::Error)]
pub enum IntentError {
    #[error("의도 저장소를 열지 못했다: {0}")]
    Open(String),
    #[error("의도 저장소 트랜잭션이 실패했다: {0}")]
    Transaction(String),
    #[error("의도 저장소 값을 풀지 못했다: {0}")]
    Decode(String),
}

/// 의도 저장소. **`intent.redb`** 다.
pub struct IntentStore {
    db: Database,
}

impl IntentStore {
    /// # Errors
    /// 파일을 열지 못하면.
    pub fn open(path: &Path) -> Result<Self, IntentError> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)
                .map_err(|e| IntentError::Open(format!("{}: {e}", dir.display())))?;
        }
        let db = Database::create(path)
            .map_err(|e| IntentError::Open(format!("{}: {e}", path.display())))?;
        Ok(Self { db })
    }

    /// 결박을 남긴다. **덮어쓰기는 같은 내용의 재기록뿐이다** —
    /// [`BindingId`] 가 `(대상, 조각)` 에서 유도되므로 다른 내용은 다른 키가 된다.
    ///
    /// # Errors
    /// 쓰기가 실패하면.
    pub fn record(&self, binding: &Binding) -> Result<(), IntentError> {
        let raw =
            postcard::to_allocvec(binding).map_err(|e| IntentError::Decode(e.to_string()))?;
        let write = self.db.begin_write().map_err(|e| IntentError::Transaction(e.to_string()))?;
        {
            let mut t =
                write.open_table(BINDING).map_err(|e| IntentError::Transaction(e.to_string()))?;
            t.insert(binding.id.as_str(), raw)
                .map_err(|e| IntentError::Transaction(e.to_string()))?;
            let mut idx = write
                .open_multimap_table(BOUND_BY)
                .map_err(|e| IntentError::Transaction(e.to_string()))?;
            idx.insert(binding.target.as_bytes().as_slice(), binding.id.as_str())
                .map_err(|e| IntentError::Transaction(e.to_string()))?;
        }
        write.commit().map_err(|e| IntentError::Transaction(e.to_string()))?;
        Ok(())
    }

    /// 이 심볼에 걸린 것 전부. **`touch` 의 근간이다.**
    ///
    /// # Errors
    /// 읽기가 실패하면.
    pub fn bound_to(&self, target: SymbolId) -> Result<Vec<Binding>, IntentError> {
        let read = self.db.begin_read().map_err(|e| IntentError::Transaction(e.to_string()))?;
        let (Ok(idx), Ok(t)) = (read.open_multimap_table(BOUND_BY), read.open_table(BINDING))
        else {
            return Ok(Vec::new());
        };

        let mut out = Vec::new();
        let ids = idx
            .get(target.as_bytes().as_slice())
            .map_err(|e| IntentError::Transaction(e.to_string()))?;
        for id in ids {
            let id = id.map_err(|e| IntentError::Transaction(e.to_string()))?;
            if let Some(v) =
                t.get(id.value()).map_err(|e| IntentError::Transaction(e.to_string()))?
            {
                out.push(
                    postcard::from_bytes(&v.value())
                        .map_err(|e| IntentError::Decode(e.to_string()))?,
                );
            }
        }
        out.sort_by(|a: &Binding, b: &Binding| a.id.cmp(&b.id));
        Ok(out)
    }

    /// 하나를 이름으로.
    ///
    /// # Errors
    /// 읽기가 실패하면.
    pub fn get(&self, id: &BindingId) -> Result<Option<Binding>, IntentError> {
        let read = self.db.begin_read().map_err(|e| IntentError::Transaction(e.to_string()))?;
        let Ok(t) = read.open_table(BINDING) else {
            return Ok(None);
        };
        let got =
            t.get(id.as_str()).map_err(|e| IntentError::Transaction(e.to_string()))?;
        match got {
            None => Ok(None),
            Some(v) => Ok(Some(
                postcard::from_bytes(&v.value()).map_err(|e| IntentError::Decode(e.to_string()))?,
            )),
        }
    }

    /// 결박 전부. **`doctor` 가 그래프를 세우려면 하나가 아니라 전부가 필요하다.**
    ///
    /// # 지우는 API 가 아니다
    ///
    /// 이 파일에 지우는 함수가 없다는 것이 R-21 의 대응이고, 읽는 함수가 하나 느는 것은
    /// 그 대응을 건드리지 않는다. **결박 id 순으로 정렬한다** — 같은 저장소에서 같은
    /// 순서가 나와야 `doctor` 의 표본이 재현된다.
    ///
    /// # Errors
    /// 읽기가 실패하거나 값을 풀지 못하면.
    pub fn all(&self) -> Result<Vec<Binding>, IntentError> {
        let read = self.db.begin_read().map_err(|e| IntentError::Transaction(e.to_string()))?;
        let Ok(t) = read.open_table(BINDING) else {
            return Ok(Vec::new());
        };
        let mut out = Vec::new();
        let range = t.iter().map_err(|e| IntentError::Transaction(e.to_string()))?;
        for row in range {
            let (_, v) = row.map_err(|e: redb::StorageError| {
                IntentError::Transaction(e.to_string())
            })?;
            let binding: Binding = postcard::from_bytes(&v.value())
                .map_err(|e| IntentError::Decode(e.to_string()))?;
            out.push(binding);
        }
        out.sort_by(|a: &Binding, b: &Binding| a.id.cmp(&b.id));
        Ok(out)
    }

    /// 결박이 몇 건인가. **파생층을 지운 뒤 이 수가 그대로여야 한다**(S3 합격선 ①).
    ///
    /// # Errors
    /// 읽기가 실패하면.
    pub fn count(&self) -> Result<usize, IntentError> {
        let read = self.db.begin_read().map_err(|e| IntentError::Transaction(e.to_string()))?;
        let Ok(t) = read.open_table(BINDING) else {
            return Ok(0);
        };
        let n: u64 = t.len().map_err(|e: redb::StorageError| IntentError::Transaction(e.to_string()))?;
        Ok(usize::try_from(n).unwrap_or(usize::MAX))
    }
}
