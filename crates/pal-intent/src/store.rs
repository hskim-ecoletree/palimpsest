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
    Database, MultimapTableDefinition, ReadOnlyDatabase, ReadableDatabase, ReadableMultimapTable,
    ReadableTable, ReadableTableMetadata, TableDefinition,
};

/// 결박 실체.
const BINDING: TableDefinition<&str, Vec<u8>> = TableDefinition::new("binding");

/// 대상 심볼 → 결박들. **역방향 색인** — `touch` 가 이것을 읽는다.
const BOUND_BY: MultimapTableDefinition<&[u8], &str> = MultimapTableDefinition::new("bound_by");

/// **감시 원소** → 결박들. `BOUND_BY` 와 다르다 — 반경이 `symbol` 보다 넓으면 갈린다.
///
/// # 왜 여기이고 2층이 아닌가 (`[f09].watch_placement`)
///
/// `[f05].bound_by_placement` 가 이 둘을 2층에 **안 세웠고** 근거가 *"`Projection::rebuild`
/// 가 지우는데 재생 코드가 없다"* 였다. **그 근거가 의도 저장소 안에서는 안 걸린다** —
/// 같은 파일이라 2층 재구축이 안 건드리고, 재생 경로가 `BINDING` 훑기 **하나**다.
/// F05 의 결정을 뒤집는 것이 아니라 **그 결정이 닿지 않는 자리**이고, `[f05.2]` ④ 의
/// 모집단(2층 테이블)이 **안 는다**.
///
/// # 이것이 없으면 증분 갱신이 없다
///
/// 재추출이 digest 를 바꾼 심볼 집합을 알 때(F04), **어느 결박을 다시 계산해야 하는지**
/// 알 길이 `BINDING` 전수 훑기밖에 없다. 반경이 `symbol` 이면 `BOUND_BY` 로 충분했고,
/// **반경을 들이는 순간 둘이 갈린다**(F09 §4.1).
const WATCH: MultimapTableDefinition<&[u8], &str> = MultimapTableDefinition::new("watch");

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
    /// # 셋이 한 트랜잭션에서 움직인다
    ///
    /// `BINDING` · `BOUND_BY` · `WATCH`. **셋 중 하나만 쓰이는 경로가 있으면 그것이 곧
    /// 조용한 유실이다** — 색인이 실체와 갈려도 **결박 건수는 안 변하므로 왕복 검사가
    /// 통과한다**(F04 가 발견한 형태). `[f09.1.pass]` 가 그 갈림을 전수로 잰다.
    ///
    /// # `WATCH` 의 낡은 자리를 지우는 것은 「의도를 지우는 것」이 아니다
    ///
    /// 같은 결박을 더 좁은 반경으로 다시 걸면 옛 감시 원소가 `WATCH` 에 남는다. 그것을
    /// 안 지우면 색인이 실체와 갈리고 위 검사가 반증을 낸다. **지우는 것은 색인의 한
    /// 줄이지 결박이 아니다** — `pal-intent` 에 **지우는 공개 API 가 없다**는 R-21 의
    /// 대응은 그대로다(S3 합격선 ⑤ 는 `pub fn` 의 이름을 센다).
    pub fn record(&self, binding: &Binding) -> Result<(), IntentError> {
        let raw =
            postcard::to_allocvec(binding).map_err(|e| IntentError::Decode(e.to_string()))?;
        // **덮어쓰기 전의 감시 집합**을 먼저 읽는다 — 트랜잭션 밖에서 읽어도 되는 이유는
        // 쓰기가 한 프로세스뿐이기 때문이다(`redb` 의 배타 락).
        let 옛_감시: Vec<SymbolId> =
            self.get(&binding.id)?.map(|b| b.watch.iter().map(|w| w.symbol).collect()).unwrap_or_default();
        let 새_감시: Vec<SymbolId> = binding.watch.iter().map(|w| w.symbol).collect();

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

            let mut watch = write
                .open_multimap_table(WATCH)
                .map_err(|e| IntentError::Transaction(e.to_string()))?;
            for s in &옛_감시 {
                if !새_감시.contains(s) {
                    watch
                        .remove(s.as_bytes().as_slice(), binding.id.as_str())
                        .map_err(|e| IntentError::Transaction(e.to_string()))?;
                }
            }
            for s in &새_감시 {
                watch
                    .insert(s.as_bytes().as_slice(), binding.id.as_str())
                    .map_err(|e| IntentError::Transaction(e.to_string()))?;
            }
        }
        write.commit().map_err(|e| IntentError::Transaction(e.to_string()))?;
        Ok(())
    }

    /// **이 심볼들을 지켜보는** 결박 전부 — 증분 상태 갱신의 입구 (F09 §4.1).
    ///
    /// > 재추출 시 digest 가 바뀐 심볼 집합을 알고 있으므로(F04), `WATCH` 테이블의
    /// > 역방향 조회로 **영향받는 결박만** 재계산한다. 전체 재계산이 아니다.
    ///
    /// **결박 id 순으로 정렬한다** — 같은 저장소가 같은 순서를 낸다.
    ///
    /// # Errors
    /// 읽기가 실패하거나 값을 풀지 못하면.
    pub fn bindings_watching(&self, changed: &[SymbolId]) -> Result<Vec<Binding>, IntentError> {
        let Some(read) = self.read()? else { return Ok(Vec::new()) };
        let (Ok(watch), Ok(t)) = (read.open_multimap_table(WATCH), read.open_table(BINDING)) else {
            return Ok(Vec::new());
        };
        let mut ids: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        for s in changed {
            let got = watch
                .get(s.as_bytes().as_slice())
                .map_err(|e| IntentError::Transaction(e.to_string()))?;
            for id in got {
                ids.insert(
                    id.map_err(|e| IntentError::Transaction(e.to_string()))?.value().to_owned(),
                );
            }
        }
        let mut out = Vec::new();
        for id in ids {
            if let Some(v) = t
                .get(id.as_str())
                .map_err(|e| IntentError::Transaction(e.to_string()))?
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

    /// `WATCH` 색인에 실제로 적힌 `(감시 원소, 결박 id)` 전부 — **검사가 이것을 쓴다.**
    ///
    /// # 이 함수가 존재하는 이유가 곧 이 색인의 위험이다
    ///
    /// 같은 사실이 `BINDING` 과 `WATCH` **두 곳**에 적혀 있다. 갈려도 **결박 건수는
    /// 안 변하므로 왕복 검사가 통과한다** — 그러므로 *"두 곳이 갈리는지를 세는 것"* 이
    /// 유일한 검사이고, 그 검사가 읽을 자리가 여기다(`[f09.1.pass]`).
    ///
    /// # Errors
    /// 읽기가 실패하면.
    pub fn watch_index(&self) -> Result<Vec<(SymbolId, BindingId)>, IntentError> {
        let Some(read) = self.read()? else { return Ok(Vec::new()) };
        let Ok(watch) = read.open_multimap_table(WATCH) else { return Ok(Vec::new()) };
        let mut out = Vec::new();
        for row in watch.iter().map_err(|e| IntentError::Transaction(e.to_string()))? {
            let (k, vs) = row.map_err(|e: redb::StorageError| {
                IntentError::Transaction(e.to_string())
            })?;
            let mut raw = [0u8; 32];
            let bytes = k.value();
            if bytes.len() != 32 {
                return Err(IntentError::Decode(format!(
                    "감시 색인의 열쇠가 32바이트가 아니다: {}바이트",
                    bytes.len()
                )));
            }
            raw.copy_from_slice(bytes);
            let symbol = SymbolId::from_bytes(raw);
            for v in vs {
                let v = v.map_err(|e| IntentError::Transaction(e.to_string()))?;
                out.push((symbol, BindingId::new(v.value())));
            }
        }
        out.sort();
        Ok(out)
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
///
/// # 1 → 2 (F09 · 2026-08-14)
///
/// `Binding` 이 셋을 더 실었다 — `subject`([`pal_core::EntityId`]) · `radius` ·
/// `bound_at_time`. **판 1 파일은 그 셋이 없다.**
///
/// **판 1 을 읽을 수 있어야 한다.** 못 읽으면 그것은 유실이고, 이 저장소에서
/// **재생 불가능한 유일한 데이터**다([R-21]). 올리는 규칙은 [`올린다`] 에 있다.
pub const JSONL_SCHEMA_VERSION: u32 = 2;

/// 이 빌드가 **읽을 수 있는** 판들. 내보내기는 언제나 최신이다.
pub const READABLE_SCHEMA_VERSIONS: &[u32] = &[1, 2];

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
    /// **어느 판을 읽었나.** 옛 판을 읽었다는 사실이 산출에 실려야 사용자가 한 번 더
    /// 내보내 판을 올릴 수 있다.
    pub schema_version: u32,
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
        let version = match serde_json::from_str::<IntentLine>(head) {
            Ok(IntentLine::Header { schema_version })
                if READABLE_SCHEMA_VERSIONS.contains(&schema_version) =>
            {
                schema_version
            }
            Ok(IntentLine::Header { schema_version }) => {
                return Err(IntentError::Decode(format!(
                    "판이 다르다 — 파일 {schema_version} · 이 빌드가 읽는 것 {READABLE_SCHEMA_VERSIONS:?}"
                )));
            }
            _ => {
                return Err(IntentError::Decode(
                    "첫 줄이 머리가 아니다 — 판을 모르고 읽으면 조용히 잘못 읽는다".to_owned(),
                ));
            }
        };

        let mut report = ImportReport { schema_version: version, ..ImportReport::default() };
        for (n, line) in lines.enumerate() {
            let 줄 = n + 2;
            // **판마다 읽는 모양이 다르다.** 새 모양으로 옛 파일을 읽으려 하면
            // `serde` 가 *"필드가 없다"* 로 멈추고, 그 멈춤이 곧 유실이다.
            let parsed: IntentLine = if version == 1 {
                let v1: IntentLineV1 = serde_json::from_str(line)
                    .map_err(|e| IntentError::Decode(format!("{줄}번째 줄 (판 1): {e}")))?;
                올린다(v1, 줄)?
            } else {
                serde_json::from_str(line)
                    .map_err(|e| IntentError::Decode(format!("{줄}번째 줄: {e}")))?
            };
            match parsed {
                IntentLine::Header { .. } => {
                    return Err(IntentError::Decode(format!("{줄}번째 줄에 머리가 또 있다")));
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

// ─────────────────────────────────────────────────────────────────────────────
// 판 1 — **읽을 수 있어야 한다. 못 읽으면 유실이다** ([R-21])
// ─────────────────────────────────────────────────────────────────────────────

/// 판 1 의 결박 — `subject` · `radius` · `bound_at_time` 이 없다.
#[derive(Debug, Clone, Deserialize)]
struct BindingV1 {
    id: String,
    target: pal_core::SymbolId,
    note: String,
    bound_at: pal_core::Snapshot,
    watch: Vec<pal_core::WatchEntry>,
}

/// 판 1 의 한 줄.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum IntentLineV1 {
    Header { schema_version: u32 },
    Binding(Box<BindingV1>),
    Alias(pal_core::RepoAlias),
}

/// 판 1 을 판 2 로 올린다.
///
/// # `subject` 를 **유도한다** — 새로 뽑지 않는다
///
/// [`pal_core::EntityId::mint`] 를 부르면 같은 파일을 두 번 읽을 때 **개체가 둘이
/// 된다.** 읽기는 더하기이지 바꿔치기가 아니므로(`[f05.4]` ②) 두 번 읽는 것은 정상
/// 경로이고, 그때 왕복이 항등이 아니게 된다.
///
/// 그래서 결박 id 에서 **결정적으로** 유도한다. 새 개체는 [`pal_core::EntityId::mint`]
/// 로만 태어나고, **여기는 옛 파일을 올리는 자리이지 개체를 만드는 자리가 아니다.**
///
/// # 반경은 `symbol` 이다
///
/// 판 1 에는 반경이 없었고 그때 감시 집합은 **언제나 대상 하나**였다. `symbol` 로
/// 올리는 것은 추측이 아니라 **그 판의 사실을 적는 것**이다.
///
/// # 시각은 [`pal_core::BoundTime::Worktree`] 가 아니다
///
/// 판 1 은 시각을 **안 실었다.** 「워킹트리라 없다」와 「옛 판이라 모른다」는 다른
/// 사건이므로 [`pal_core::BoundTime::Unrecorded`] 로 적는다 — 조용히 0 을 넣으면
/// *"1970년 코드 기준"* 이 화면에 뜬다.
fn 올린다(line: IntentLineV1, 줄: usize) -> Result<IntentLine, IntentError> {
    Ok(match line {
        IntentLineV1::Header { schema_version } => IntentLine::Header { schema_version },
        IntentLineV1::Alias(a) => IntentLine::Alias(a),
        IntentLineV1::Binding(b) => {
            let id = BindingId::new(b.id);
            if id.as_str().is_empty() {
                return Err(IntentError::Decode(format!("{줄}번째 줄의 결박에 id 가 없다")));
            }
            IntentLine::Binding(Box::new(Binding::from_v1(
                id,
                b.target,
                &b.note,
                b.bound_at,
                b.watch,
            )))
        }
    })
}
