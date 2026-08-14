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
    Database, MultimapTableDefinition, ReadOnlyDatabase, ReadableDatabase, ReadableTable,
    ReadableTableMetadata, TableDefinition,
};

/// 결박 실체.
const BINDING: TableDefinition<&str, Vec<u8>> = TableDefinition::new("binding");

/// 대상 심볼 → 결박들. **역방향 색인** — `touch` 가 이것을 읽는다.
const BOUND_BY: MultimapTableDefinition<&[u8], &str> = MultimapTableDefinition::new("bound_by");

#[derive(Debug, thiserror::Error)]
pub enum IntentError {
    #[error(
        "의도 저장소를 열지 못했다: {0}\n\
         **조용히 빈 채로 열지 않는다** — 결박이 0 건인 것과 파일이 깨진 것은 다른 사건이다.\n\
         복구는 JSONL 내보내기에서 한다: `pal intent import <파일>`"
    )]
    Open(String),
    #[error("의도 저장소 트랜잭션이 실패했다: {0}")]
    Transaction(String),
    #[error("의도 저장소 값을 풀지 못했다: {0}")]
    Decode(String),
}

/// 어떻게 열렸는가 — **읽기만 하는 명령은 파일을 안 써야 한다.**
///
/// # ⚠ F04 가 여기로 넘긴 것이다
///
/// `redb::Database::create` 는 **열기만 해도 파일을 쓴다.** F04 가 쟀다 — 같은
/// `pal touch` 를 두 번 돌리면 1,056,768 바이트 중 **110 바이트**가 달라진다.
/// 그래서 F04 의 재구축 등가성 시험은 *"의도가 안 변했다"* 를 **바이트가 아니라 값으로**
/// 재야 했고, 그 사실을 게이트에 1급으로 적고 F05 로 넘겼다.
///
/// `ReadOnlyDatabase::open` 은 `try_lock_shared` 를 쓰고 **파일을 쓰지 않는다.**
/// 그래서 읽기 경로가 이쪽으로 온다(`[f05.4.pass]` ④).
///
/// # `Absent` 가 변형인 이유
///
/// **파일이 없는 것과 깨진 것은 다른 사건이다.** 없으면 결박이 0 건이고 그것은 정상
/// 상태다(아직 아무도 안 걸었다). 깨졌으면 **오류다** — F05 §6 이 못 박았다:
/// *"사용자에게 유실 범위를 명시 — **조용히 빈 채로 열지 않는다**"*.
enum Handle {
    /// 파일이 아직 없다. 읽으면 전부 비어 있고, 그것이 정확한 값이다.
    Absent,
    /// 읽기 전용 — **파일을 안 쓴다.**
    Reading(ReadOnlyDatabase),
    /// 쓸 수 있다.
    Writing(Database),
}

/// 의도 저장소. **`intent.redb`** 다.
pub struct IntentStore {
    db: Handle,
}

impl IntentStore {
    /// **쓸 수 있게** 연다. 없으면 만든다.
    ///
    /// # Errors
    /// 파일을 열지 못하면.
    pub fn open(path: &Path) -> Result<Self, IntentError> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)
                .map_err(|e| IntentError::Open(format!("{}: {e}", dir.display())))?;
        }
        let db = Database::create(path)
            .map_err(|e| IntentError::Open(format!("{}: {e}", path.display())))?;
        Ok(Self { db: Handle::Writing(db) })
    }

    /// **읽기만** 한다 — 파일을 안 쓴다(`[f05.4.pass]` ④).
    ///
    /// 파일이 없으면 [`Handle::Absent`] 다. **깨졌으면 오류다** — 빈 것으로 열면
    /// *"결박 0 건"* 이 나오고 그것이 F05 §6 이 금한 형태다.
    ///
    /// # Errors
    /// 파일이 있는데 열지 못하면 — **깨졌다는 뜻이고 조용히 넘기지 않는다.**
    pub fn open_read_only(path: &Path) -> Result<Self, IntentError> {
        if !path.exists() {
            return Ok(Self { db: Handle::Absent });
        }
        let db = ReadOnlyDatabase::open(path)
            .map_err(|e| IntentError::Open(format!("{}: {e}", path.display())))?;
        Ok(Self { db: Handle::Reading(db) })
    }

    /// 읽기 트랜잭션 하나. **없는 저장소는 `None` 이고 그것이 정확한 값이다.**
    fn read(&self) -> Result<Option<redb::ReadTransaction>, IntentError> {
        match &self.db {
            Handle::Absent => Ok(None),
            Handle::Reading(db) => db.begin_read().map(Some).map_err(|e| IntentError::Transaction(e.to_string())),
            Handle::Writing(db) => db.begin_read().map(Some).map_err(|e| IntentError::Transaction(e.to_string())),
        }
    }

    /// 쓰기 트랜잭션 하나. **읽기로 연 저장소에는 없다.**
    fn write(&self) -> Result<redb::WriteTransaction, IntentError> {
        match &self.db {
            Handle::Writing(db) => {
                db.begin_write().map_err(|e| IntentError::Transaction(e.to_string()))
            }
            Handle::Absent | Handle::Reading(_) => Err(IntentError::Transaction(
                "읽기로 연 의도 저장소에 쓰려 했다".to_owned(),
            )),
        }
    }

    /// 결박을 남긴다. **덮어쓰기는 같은 내용의 재기록뿐이다** —
    /// [`BindingId`] 가 `(대상, 조각)` 에서 유도되므로 다른 내용은 다른 키가 된다.
    ///
    /// # Errors
    /// 쓰기가 실패하면.
    pub fn record(&self, binding: &Binding) -> Result<(), IntentError> {
        let raw =
            postcard::to_allocvec(binding).map_err(|e| IntentError::Decode(e.to_string()))?;
        let write = self.write()?;
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
        let Some(read) = self.read()? else { return Ok(Vec::new()) };
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
        let Some(read) = self.read()? else { return Ok(None) };
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
        let Some(read) = self.read()? else { return Ok(Vec::new()) };
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
        // **파일이 없으면 0 이고 그것이 정확한 값이다** — 아직 아무도 안 걸었다.
        // 깨진 경우는 여기 못 온다(`open_read_only` 가 오류를 낸다).
        let Some(read) = self.read()? else { return Ok(0) };
        let Ok(t) = read.open_table(BINDING) else {
            return Ok(0);
        };
        let n: u64 = t.len().map_err(|e: redb::StorageError| IntentError::Transaction(e.to_string()))?;
        Ok(usize::try_from(n).unwrap_or(usize::MAX))
    }
}

/// 저장소 별칭 — **사람이 선언한 재배치.** `RepoAlias::was` 가 열쇠다.
const ALIAS: TableDefinition<&str, Vec<u8>> = TableDefinition::new("repo_alias");

impl IntentStore {
    /// 별칭 하나를 남긴다.
    ///
    /// # 왜 여기인가 ([R-21] · F03 §7)
    ///
    /// *"이 저장소가 저 저장소였다"* 는 **코드에서 유도되지 않는다.** 파생층에 두면
    /// *"2층을 지우고 재구축"* 이 그 선언을 지우고, 재구축 등가성 검사는 그 상태에서도
    /// 통과하므로 **검사가 유실을 정상으로 승인한다.** 결박과 같은 처지이고 같은 방에 산다.
    ///
    /// # Errors
    /// 쓰기가 실패하면.
    pub fn record_alias(&self, alias: &pal_core::RepoAlias) -> Result<(), IntentError> {
        let raw = postcard::to_allocvec(alias).map_err(|e| IntentError::Decode(e.to_string()))?;
        let write = self.write()?;
        {
            let mut t =
                write.open_table(ALIAS).map_err(|e| IntentError::Transaction(e.to_string()))?;
            t.insert(alias.was.as_str(), raw)
                .map_err(|e| IntentError::Transaction(e.to_string()))?;
        }
        write.commit().map_err(|e| IntentError::Transaction(e.to_string()))?;
        Ok(())
    }

    /// 선언된 별칭 전부 — **옛 이름 순.**
    ///
    /// # Errors
    /// 읽기가 실패하거나 값을 풀지 못하면.
    pub fn aliases(&self) -> Result<Vec<pal_core::RepoAlias>, IntentError> {
        let Some(read) = self.read()? else { return Ok(Vec::new()) };
        let Ok(t) = read.open_table(ALIAS) else {
            return Ok(Vec::new());
        };
        let mut out = Vec::new();
        for row in t.iter().map_err(|e| IntentError::Transaction(e.to_string()))? {
            let (_, v) =
                row.map_err(|e: redb::StorageError| IntentError::Transaction(e.to_string()))?;
            out.push(
                postcard::from_bytes(&v.value()).map_err(|e| IntentError::Decode(e.to_string()))?,
            );
        }
        out.sort();
        Ok(out)
    }

    /// 옛 이름을 지금 이름으로 — **선언된 것만 따라간다.**
    ///
    /// # 사슬을 따라가되 **한 바퀴 이상 돌지 않는다**
    ///
    /// `a → b → a` 같은 선언은 사람이 만들 수 있고, 여기서 고칠 수 없다. **좌표를
    /// 만드는 쪽이 멈추지 않는 것**은 여기의 책임이다 — 선언 수만큼만 따라간다.
    ///
    /// # Errors
    /// 읽기가 실패하면.
    pub fn resolve_repo(&self, id: &pal_core::RepoId) -> Result<pal_core::RepoId, IntentError> {
        let all = self.aliases()?;
        let mut cursor = id.clone();
        for _ in 0..all.len() {
            let Some(next) = all.iter().find(|a| a.was == cursor) else { break };
            cursor = next.now.clone();
        }
        Ok(cursor)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// JSONL 내보내기/읽기 — **이 저장소만 지는 부담이다** (F05 §2 의 넷째 · §6 의 표)
//
// > 2층은 캐시라서 자체 구현의 최대 비용(마이그레이션·백업)이 0 이다.
// > **의도 저장소는 이 면제를 받지 못한다** — 거기만 스키마 버전과 JSONL 내보내기를 진다.
//
// > 의도 저장소 손상 → **재구축 불가.** JSONL 내보내기에서 복구.
// > **그래서 내보내기가 상시 유지된다.**
//
// **상시 유지되지 않는 내보내기는 없는 것과 같다.** 그래서 이 기능의 판정은
// 「있다」가 아니라 **「왕복이 항등이다」** 이고, CI 가 그것을 상시 돌린다.
// ─────────────────────────────────────────────────────────────────────────────

use serde::{Deserialize, Serialize};

/// 이 내보내기 형식의 판. **파일의 첫 줄이 이것을 싣는다.**
///
/// 2층에는 스키마 버전이 없다 — 지우고 다시 만들면 되기 때문이다. 여기는 **그럴 수
/// 없으므로** 버전이 있어야 하고, 없으면 옛 파일을 새 코드가 조용히 잘못 읽는다.
pub const JSONL_SCHEMA_VERSION: u32 = 1;

/// JSONL 한 줄.
///
/// **머리도 줄이다.** 별도 형식을 두면 읽는 쪽이 두 파서를 갖고, 그 둘이 갈린다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum IntentLine {
    /// 첫 줄 — **없으면 읽기를 거부한다.**
    Header { schema_version: u32 },
    /// **`Box` 인 이유**: 결박이 감시 집합을 싣고 별칭보다 훨씬 크다
    /// (`clippy::large_enum_variant`).
    Binding(Box<Binding>),
    Alias(pal_core::RepoAlias),
}

/// 읽기 한 회차의 회계.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
pub struct ImportReport {
    pub bindings: usize,
    pub aliases: usize,
    /// **이미 있던 것.** 읽기는 더하기이지 바꿔치기가 아니다 — 이 수가 그 증거다.
    pub already_present: usize,
}

impl IntentStore {
    /// 전부를 JSONL 로. **결박 id 순 → 별칭 옛 이름 순** — 같은 저장소가 같은 파일을 낸다.
    ///
    /// # Errors
    /// 읽기가 실패하거나 직렬화가 실패하면.
    pub fn export_jsonl(&self) -> Result<String, IntentError> {
        let mut out = String::new();
        let mut push = |line: &IntentLine| -> Result<(), IntentError> {
            let s = serde_json::to_string(line)
                .map_err(|e| IntentError::Decode(e.to_string()))?;
            out.push_str(&s);
            out.push('\n');
            Ok(())
        };
        push(&IntentLine::Header { schema_version: JSONL_SCHEMA_VERSION })?;
        for b in self.all()? {
            push(&IntentLine::Binding(Box::new(b)))?;
        }
        for a in self.aliases()? {
            push(&IntentLine::Alias(a))?;
        }
        Ok(out)
    }

    /// JSONL 을 읽어 **더한다.**
    ///
    /// # 이것은 바꿔치기가 아니다 (`[f05.4.pass]` ②)
    ///
    /// 이 크레이트에 **지우는 함수가 없다는 것이 R-21 의 대응**이고, 그 대응이 이
    /// 명령에서도 참이어야 한다. 파일에 없는 결박은 **그대로 남는다** — 읽기가
    /// 저장소를 파일의 모습으로 만들면 그것이 곧 지우는 경로다.
    ///
    /// # Errors
    /// 머리가 없거나 판이 다르거나, 줄을 풀지 못하거나, 쓰기가 실패하면.
    pub fn import_jsonl(&self, text: &str) -> Result<ImportReport, IntentError> {
        let mut lines = text.lines().filter(|l| !l.trim().is_empty());
        let head = lines.next().ok_or_else(|| {
            IntentError::Decode("빈 파일이다 — 머리 줄이 없으면 판을 알 수 없다".to_owned())
        })?;
        match serde_json::from_str::<IntentLine>(head) {
            Ok(IntentLine::Header { schema_version }) if schema_version == JSONL_SCHEMA_VERSION => {}
            Ok(IntentLine::Header { schema_version }) => {
                return Err(IntentError::Decode(format!(
                    "판이 다르다 — 파일 {schema_version} · 이 빌드 {JSONL_SCHEMA_VERSION}"
                )));
            }
            _ => {
                return Err(IntentError::Decode(
                    "첫 줄이 머리가 아니다 — 판을 모르고 읽으면 조용히 잘못 읽는다".to_owned(),
                ));
            }
        }

        let mut report = ImportReport::default();
        for (n, line) in lines.enumerate() {
            let parsed: IntentLine = serde_json::from_str(line)
                .map_err(|e| IntentError::Decode(format!("{}번째 줄: {e}", n + 2)))?;
            match parsed {
                IntentLine::Header { .. } => {
                    return Err(IntentError::Decode(format!("{}번째 줄에 머리가 또 있다", n + 2)));
                }
                IntentLine::Binding(b) => {
                    if self.get(&b.id)?.is_some() {
                        report.already_present += 1;
                    }
                    self.record(&b)?;
                    report.bindings += 1;
                }
                IntentLine::Alias(a) => {
                    self.record_alias(&a)?;
                    report.aliases += 1;
                }
            }
        }
        Ok(report)
    }
}
