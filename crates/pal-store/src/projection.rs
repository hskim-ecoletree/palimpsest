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
//! 2층에만 있는 상태는 존재하지 않는다. 통째로 지우고 1층 + 의도 저장소에서 재구축하면
//! **모든 자리에서** 같은 값이 나와야 한다(`[f05.2.pass]` ④ — 전수로 잰다).
//! 다르면 2층은 캐시가 아니라 원본이고, 그 순간 *"자체 구현의 최대 비용이 0"* 이라는
//! 주장이 무너진다.
//!
//! **그래서 결박 색인(`BOUND_BY`·`WATCH`)이 여기 없다.** F05 §3 은 그 둘을 2층 테이블로
//! 적었지만 §3.1 이 근거로 든 *"지워도 다시 만들 수 있다"* 의 **재생 경로가 없다.**
//! 세우면 재구축이 그것을 지우고 `touch` 가 조용히 빈 결박을 낸다 —
//! **R-21 이 「지우는 명령」이 아니라 「다시 안 만드는 재구축」으로 돌아오는 형태다.**
//! 판단 전문은 `corpus/criteria.toml` `[f05].bound_by_placement`.
//!
//! # 배치 커밋이 격리를 깨지 않는 형태 (`[f05.2.pass]` ③)
//!
//! F05 §4 는 *"파일 1,000 개 단위로 묶어 커밋"* 을 요구한다. 그런데 `[f22.4]` 가 등록한
//! 합격선은 **재구축 중 부분 갱신 관측 0/100** 이고, 한 재구축을 여러 트랜잭션으로
//! 쪼개면 읽는 쪽이 반쯤 채워진 2층을 본다. **등록된 합격선을 사후에 고치지 않는다.**
//!
//! ```text
//!   ① 배치는 무대(`*.staging`)에 커밋한다 — fsync 비용이 실제로 나뉜다
//!   ② 마지막 한 트랜잭션에서 살아 있는 자리를 지우고 무대를 이름 바꾼다
//!      → 읽는 쪽은 **옛 세대 전체** 아니면 **새 세대 전체**만 본다
//! ```
//!
//! 그리고 이 형태가 공짜로 하나를 준다 — **무대가 있으면 재구축 중이다.**
//! 봉투의 `projection.rebuild` 가 `NotBuilt{F05}` 였던 자리가 값이 된다
//! (DESIGN §12.7 격리 3번).

use std::collections::BTreeMap;
use std::path::Path;

use pal_core::{FileRow, QueryLogEntry, ReferenceEdge, RepoPath, SymbolId, SymbolNode};
use redb::{
    Database, MultimapTableDefinition, MultimapTableHandle, ReadOnlyDatabase, ReadableDatabase,
    ReadableMultimapTable, ReadableTable, ReadableTableMetadata, TableDefinition, TableHandle,
    WriteTransaction,
};

// ── 살아 있는 자리 ───────────────────────────────────────────────────────────

/// `symbol_id` → 심볼. 키가 32바이트 요약이다.
const SYMBOL: TableDefinition<&[u8], Vec<u8>> = TableDefinition::new("symbol");
/// 이름 → `symbol_id` 들. **사람은 해시로 묻지 않는다.**
const BY_NAME: MultimapTableDefinition<&str, &[u8]> = MultimapTableDefinition::new("by_name");
/// 경로 → 파일 노드.
const FILE: TableDefinition<&str, Vec<u8>> = TableDefinition::new("file");
/// `from` → `to` 들. 정방향 인접.
const EDGE_OUT: MultimapTableDefinition<&[u8], &[u8]> = MultimapTableDefinition::new("edge_out");
/// `to` → `from` 들. **역방향이 이 제품의 1순위 질의다**(F05 §3).
///
/// 저장 2배를 지불하고 *"누가 이걸 부르나"* 를 O(차수)로 만든다. 정방향만 두고 스캔하면
/// O(전체)다.
const EDGE_IN: MultimapTableDefinition<&[u8], &[u8]> = MultimapTableDefinition::new("edge_in");
/// 경로 → 그 파일의 `symbol_id` 들.
///
/// **없으면 `symbol.contains` 가 심볼 전체를 훑는다** — 10⁶ 에서 그것은 질의가 아니라
/// 전수 스캔이고, 벤치의 선형성이 거기서 무너진다.
const BY_FILE: MultimapTableDefinition<&str, &[u8]> = MultimapTableDefinition::new("by_file");
/// `(파일, 이름)` → `symbol_id`. F07 의 해소가 읽을 자리.
const EXPORTS: TableDefinition<(&str, &str), &[u8]> = TableDefinition::new("exports");
/// `(스냅샷, 순번)` → 질의 한 줄. **append-only** (F05 §5.3).
///
/// # 스티칭이 이 자리를 안 건드린다
///
/// 질의는 일어난 사건이고 1층에도 git 에도 없다. **재구축이 지우면 F17 의 입력이
/// 조용히 사라진다** — 그래서 교체 목록에 이것이 없다(`[f05.3.pass]` ②).
/// 그리고 이 파일에 **지우는 함수가 없다.**
const QUERY_LOG: TableDefinition<(&str, u64), Vec<u8>> = TableDefinition::new("query_log");
/// 이 투영이 **무엇에 대해** 세워졌는가. 재구축 대상이 아니다 — 마지막 트랜잭션이 적는다.
const META: TableDefinition<&str, String> = TableDefinition::new("meta");

// ── 무대 ─────────────────────────────────────────────────────────────────────
//
// 이름이 `.staging` 으로 끝나는 것이 곧 *"재구축 중"* 의 관측 경로다.

const SYMBOL_STAGE: TableDefinition<&[u8], Vec<u8>> = TableDefinition::new("symbol.staging");
const BY_NAME_STAGE: MultimapTableDefinition<&str, &[u8]> =
    MultimapTableDefinition::new("by_name.staging");
const FILE_STAGE: TableDefinition<&str, Vec<u8>> = TableDefinition::new("file.staging");
const EDGE_OUT_STAGE: MultimapTableDefinition<&[u8], &[u8]> =
    MultimapTableDefinition::new("edge_out.staging");
const EDGE_IN_STAGE: MultimapTableDefinition<&[u8], &[u8]> =
    MultimapTableDefinition::new("edge_in.staging");
const BY_FILE_STAGE: MultimapTableDefinition<&str, &[u8]> =
    MultimapTableDefinition::new("by_file.staging");
const EXPORTS_STAGE: TableDefinition<(&str, &str), &[u8]> =
    TableDefinition::new("exports.staging");

/// 무대 이름의 꼬리. **이것이 있으면 재구축 중이다.**
const STAGE_SUFFIX: &str = ".staging";

/// `META` 안의 열쇠 — 이 투영이 선 스냅샷.
const META_BUILT_FOR: &str = "built_for";

#[derive(Debug, thiserror::Error)]
pub enum ProjectionError {
    #[error("2층을 열지 못했다: {0}")]
    Open(String),
    #[error("2층 트랜잭션이 실패했다: {0}")]
    Transaction(String),
    #[error("2층 값을 풀지 못했다: {0}")]
    Decode(String),
    /// **읽기 전용으로 붙었는데 쓰려 했다.**
    ///
    /// 조용히 무시하지 않는 이유: 무시하면 스티칭이 아무것도 안 하고 성공했다고
    /// 말하고, 그 뒤의 질의는 **빈 2층 위에서** 답한다. 그 답은 비어 있고 정직해
    /// 보이지만 실제로는 이 빌드가 자기 인덱스를 안 세운 것이다.
    #[error("2층에 읽기 전용으로 붙었다 — 쓸 수 없다")]
    ReadOnly,
}

fn tx(e: impl std::fmt::Display) -> ProjectionError {
    ProjectionError::Transaction(e.to_string())
}

/// 파일 하나치의 스티칭 입력 — **1패스가 파일마다 만드는 것**(F05 §4).
///
/// **파일 하나만 보고 만들어진다.** 그것이 1패스가 병렬 가능하고 배치로 끊을 수 있는
/// 이유이고, 파일 간 해소(F07)가 여기 없는 이유다.
#[derive(Debug, Clone)]
pub struct FileStitch {
    pub file: FileRow,
    pub symbols: Vec<SymbolNode>,
    /// 이 파일이 이름으로 내보내는 것 — `(이름, 심볼)`.
    pub exports: Vec<(String, SymbolId)>,
    /// 파일 **안**에서 해소된 참조.
    pub edges: Vec<ReferenceEdge>,
}

/// 스티칭 한 회차의 회계. **커밋 수가 여기 있는 이유는 `[f05.2.pass]` ③ 이다** —
/// 배치를 넣었다는 주장은 커밋 수를 세지 않으면 검사되지 않는다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize)]
pub struct StitchReport {
    pub files: usize,
    pub symbols: usize,
    pub edges: usize,
    pub exports: usize,
    /// **배치 커밋의 수.** 이 값이 없으면 *"배치를 넣었다"* 는 주장뿐이다
    /// (`[f05.2.pass]` ③).
    pub batch_commits: usize,
    /// 전체 커밋 수 — 무대 준비 1 + 배치 N + 교체 1.
    pub commits: usize,
}

/// 2층에 어떻게 붙었는가 — **락이 다르다.**
///
/// `redb` 4.1 의 실물: [`Database::create`]·[`Database::open`] 은 **배타** 락이고
/// [`ReadOnlyDatabase::open`] 만 **공유** 락이다. F05 §6 의 표가
/// *"읽기는 동시 가능, 쓰기는 하나. **CLI 는 읽기 전용으로 붙는다**"* 라고 적었는데
/// **여는 방법이 하나뿐이라 그 문장이 성립하지 않았다** — 두 프로세스가 동시에
/// `pal query` 를 돌리면 `Database already open. Cannot acquire lock.` 이 났다.
enum Attached {
    /// 쓸 수 있다. **한 프로세스만.**
    Writable(Database),
    /// 읽기만. **여럿이 동시에 붙는다.**
    ReadOnly(ReadOnlyDatabase),
}

/// 질의 투영. **`index.redb` 다** — 의도 저장소와 파일이 갈려 있다(R-21).
pub struct Projection {
    db: Attached,
}

impl Projection {
    /// 읽기 트랜잭션 하나. **여는 자리가 하나다.**
    fn read(&self) -> Result<redb::ReadTransaction, ProjectionError> {
        match &self.db {
            Attached::Writable(d) => d.begin_read().map_err(tx),
            Attached::ReadOnly(d) => d.begin_read().map_err(tx),
        }
    }

    /// 쓰기 트랜잭션 하나. **여는 자리가 하나다.**
    ///
    /// 읽기 전용으로 붙었으면 **거절한다.** 조용히 무시하면 스티칭이 아무것도 안 하고
    /// 성공했다고 말하고, 그 뒤의 질의는 빈 2층 위에서 답한다.
    fn write(&self) -> Result<WriteTransaction, ProjectionError> {
        match &self.db {
            Attached::Writable(d) => d.begin_write().map_err(tx),
            Attached::ReadOnly(_) => Err(ProjectionError::ReadOnly),
        }
    }

    /// **쓸 수 있게** 붙는다 — 배타 락이다. 파일이 없으면 만든다.
    ///
    /// # Errors
    /// 파일을 열지 못하거나 **다른 프로세스가 이미 쓰기로 붙어 있으면**.
    pub fn open(path: &Path) -> Result<Self, ProjectionError> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)
                .map_err(|e| ProjectionError::Open(format!("{}: {e}", dir.display())))?;
        }
        let db = Database::create(path)
            .map_err(|e| ProjectionError::Open(format!("{}: {e}", path.display())))?;
        Ok(Self { db: Attached::Writable(db) })
    }

    /// **읽기만** 붙는다 — 공유 락이라 여럿이 동시에 붙는다.
    ///
    /// # 없으면 만들지 않는다 — 그리고 조용히 쓰기로 안 돌아간다
    ///
    /// 2층이 아직 안 세워졌는데 읽기 전용으로 붙으면 **실패한다.** 여기서 슬쩍
    /// [`Self::open`] 으로 되돌아가면 `--read-only` 가 거짓말이 되고, 부르는 쪽은
    /// 자기가 배타 락을 쥐었다는 것을 모른다.
    ///
    /// # Errors
    /// 파일이 없거나, 깨끗이 닫히지 않아 복구가 필요하거나, 읽지 못하면.
    pub fn open_read_only(path: &Path) -> Result<Self, ProjectionError> {
        let db = ReadOnlyDatabase::open(path)
            .map_err(|e| ProjectionError::Open(format!("{}: {e}", path.display())))?;
        Ok(Self { db: Attached::ReadOnly(db) })
    }

    /// 읽기 전용으로 붙었는가. **값이다** — 질의 로그를 못 남긴 사유가 여기서 나온다.
    #[must_use]
    pub const fn is_read_only(&self) -> bool {
        matches!(self.db, Attached::ReadOnly(_))
    }

    /// **1패스 스티칭.** 무대에 배치로 쓰고 마지막 한 트랜잭션에서 교체한다.
    ///
    /// `batch_files` 는 인자다 — 상수면 시험이 배치를 두 번 이상 내지 못하고,
    /// 그러면 *"배치를 넣었다"* 가 검사되지 않는다(`[f05.2.pass]` ③).
    ///
    /// # Errors
    /// 쓰기가 실패하거나 값을 담지 못하면.
    pub fn stitch(
        &self,
        built_for: &str,
        files: &[FileStitch],
        batch_files: usize,
    ) -> Result<StitchReport, ProjectionError> {
        let batch = batch_files.max(1);
        let mut report = StitchReport::default();

        // ① 무대를 비운다. **중단된 앞 회차의 찌꺼기가 남을 수 있다.**
        //    그리고 여기서 자리를 만들어 둔다 — 파일이 0 개여도 교체가 성립해야 한다.
        self.prepare_stage()?;
        report.commits += 1;

        // ② 배치마다 커밋한다. **중단되면 그 배치만 잃고 살아 있는 자리는 온전하다.**
        for chunk in files.chunks(batch) {
            let write = self.write()?;
            {
                let mut by_id = write.open_table(SYMBOL_STAGE).map_err(tx)?;
                let mut by_name = write.open_multimap_table(BY_NAME_STAGE).map_err(tx)?;
                let mut files_t = write.open_table(FILE_STAGE).map_err(tx)?;
                let mut out = write.open_multimap_table(EDGE_OUT_STAGE).map_err(tx)?;
                let mut into = write.open_multimap_table(EDGE_IN_STAGE).map_err(tx)?;
                let mut by_file = write.open_multimap_table(BY_FILE_STAGE).map_err(tx)?;
                let mut exports = write.open_table(EXPORTS_STAGE).map_err(tx)?;

                for f in chunk {
                    let raw = postcard::to_allocvec(&f.file)
                        .map_err(|e| ProjectionError::Decode(e.to_string()))?;
                    files_t.insert(f.file.path.as_str(), raw).map_err(tx)?;
                    report.files += 1;

                    for s in &f.symbols {
                        let raw = postcard::to_allocvec(s)
                            .map_err(|e| ProjectionError::Decode(e.to_string()))?;
                        by_id.insert(s.id.as_bytes().as_slice(), raw).map_err(tx)?;
                        by_name
                            .insert(s.name.as_str(), s.id.as_bytes().as_slice())
                            .map_err(tx)?;
                        by_file
                            .insert(f.file.path.as_str(), s.id.as_bytes().as_slice())
                            .map_err(tx)?;
                        report.symbols += 1;
                    }
                    for (name, id) in &f.exports {
                        exports
                            .insert((f.file.path.as_str(), name.as_str()), id.as_bytes().as_slice())
                            .map_err(tx)?;
                        report.exports += 1;
                    }
                    for e in &f.edges {
                        // **양방향을 함께 쓴다.** 한쪽만 쓰면 행 수가 갈리고,
                        // `[f05.2.pass]` ① 이 그것을 잡는다.
                        out.insert(e.from.as_bytes().as_slice(), e.to.as_bytes().as_slice())
                            .map_err(tx)?;
                        into.insert(e.to.as_bytes().as_slice(), e.from.as_bytes().as_slice())
                            .map_err(tx)?;
                        report.edges += 1;
                    }
                }
            }
            write.commit().map_err(tx)?;
            report.commits += 1;
            report.batch_commits += 1;
        }

        // ③ **한 트랜잭션에서 교체한다.** 읽는 쪽은 옛 세대 전체 아니면 새 세대 전체다.
        self.swap(built_for)?;
        report.commits += 1;

        Ok(report)
    }

    /// 무대를 살아 있는 자리로 **한 트랜잭션에 갈아 끼운다.**
    ///
    /// 여기가 `[f22.4]` 의 0/100 이 배치 커밋과 함께 서는 자리다 — 읽는 쪽이 볼 수 있는
    /// 상태는 **교체 전 전체**와 **교체 후 전체** 둘뿐이다.
    fn swap(&self, built_for: &str) -> Result<(), ProjectionError> {
        let write = self.write()?;
        write.delete_table(SYMBOL).map_err(tx)?;
        write.delete_multimap_table(BY_NAME).map_err(tx)?;
        write.delete_table(FILE).map_err(tx)?;
        write.delete_multimap_table(EDGE_OUT).map_err(tx)?;
        write.delete_multimap_table(EDGE_IN).map_err(tx)?;
        write.delete_multimap_table(BY_FILE).map_err(tx)?;
        write.delete_table(EXPORTS).map_err(tx)?;

        write.rename_table(SYMBOL_STAGE, SYMBOL).map_err(tx)?;
        write.rename_multimap_table(BY_NAME_STAGE, BY_NAME).map_err(tx)?;
        write.rename_table(FILE_STAGE, FILE).map_err(tx)?;
        write.rename_multimap_table(EDGE_OUT_STAGE, EDGE_OUT).map_err(tx)?;
        write.rename_multimap_table(EDGE_IN_STAGE, EDGE_IN).map_err(tx)?;
        write.rename_multimap_table(BY_FILE_STAGE, BY_FILE).map_err(tx)?;
        write.rename_table(EXPORTS_STAGE, EXPORTS).map_err(tx)?;

        {
            let mut meta = write.open_table(META).map_err(tx)?;
            meta.insert(META_BUILT_FOR, built_for.to_owned()).map_err(tx)?;
        }
        write.commit().map_err(tx)?;
        Ok(())
    }

    /// 무대를 비우고 자리를 만든다.
    fn prepare_stage(&self) -> Result<(), ProjectionError> {
        let write = self.write()?;
        clear_stage(&write)?;
        open_stage(&write)?;
        write.commit().map_err(tx)?;
        Ok(())
    }

    /// 심볼만으로 되세운다 — **파일 노드도 엣지도 모르는 부르는 쪽을 위한 자리.**
    ///
    /// # 이것도 무대를 거친다
    ///
    /// 살아 있는 자리에 바로 쓰면 읽는 쪽이 **반쯤 채워진 2층**을 본다. 옛 판은 한
    /// 트랜잭션이라 그 문제가 없었고, 배치 커밋을 넣는 순간 생긴다 —
    /// `pal-store/tests/isolation.rs` 가 그것을 100 회로 잰다.
    ///
    /// # Errors
    /// 쓰기가 실패하면.
    pub fn rebuild(&self, symbols: &[SymbolNode]) -> Result<usize, ProjectionError> {
        self.prepare_stage()?;
        {
            let write = self.write()?;
            {
                let mut by_id = write.open_table(SYMBOL_STAGE).map_err(tx)?;
                let mut by_name = write.open_multimap_table(BY_NAME_STAGE).map_err(tx)?;
                for s in symbols {
                    let raw = postcard::to_allocvec(s)
                        .map_err(|e| ProjectionError::Decode(e.to_string()))?;
                    by_id.insert(s.id.as_bytes().as_slice(), raw).map_err(tx)?;
                    by_name.insert(s.name.as_str(), s.id.as_bytes().as_slice()).map_err(tx)?;
                }
            }
            write.commit().map_err(tx)?;
        }
        self.swap("")?;
        Ok(symbols.len())
    }

    /// 좌표로 심볼 하나.
    ///
    /// # Errors
    /// 읽기가 실패하거나 값을 풀지 못하면.
    pub fn symbol(&self, id: SymbolId) -> Result<Option<SymbolNode>, ProjectionError> {
        let read = self.read()?;
        // 아직 아무것도 안 들어간 2층이면 자리 자체가 없다.
        let Ok(by_id) = read.open_table(SYMBOL) else {
            return Ok(None);
        };
        let got = by_id.get(id.as_bytes().as_slice()).map_err(tx)?;
        match got {
            None => Ok(None),
            Some(v) => Ok(Some(
                postcard::from_bytes(&v.value())
                    .map_err(|e| ProjectionError::Decode(e.to_string()))?,
            )),
        }
    }

    /// 이름으로 후보를 찾는다. **여럿일 수 있고, 그것이 정상이다.**
    ///
    /// # Errors
    /// 읽기가 실패하면.
    pub fn resolve_name(&self, name: &str) -> Result<Vec<SymbolNode>, ProjectionError> {
        let read = self.read()?;
        let (Ok(by_name), Ok(by_id)) = (read.open_multimap_table(BY_NAME), read.open_table(SYMBOL))
        else {
            return Ok(Vec::new());
        };

        let mut out: Vec<SymbolNode> = Vec::new();
        let ids = by_name.get(name).map_err(tx)?;
        for id in ids {
            let id = id.map_err(tx)?;
            if let Some(v) = by_id.get(id.value()).map_err(tx)? {
                out.push(
                    postcard::from_bytes(&v.value())
                        .map_err(|e| ProjectionError::Decode(e.to_string()))?,
                );
            }
        }
        // **결정적 순서** — 같은 질의가 같은 순서를 내야 산출을 비교할 수 있다.
        out.sort_by(|a, b| {
            a.path.cmp(&b.path).then_with(|| a.span.line_start.cmp(&b.span.line_start))
        });
        Ok(out)
    }

    /// 2층에 들어 있는 심볼 수. 봉투의 `projection.symbols_indexed` 가 이것이다.
    ///
    /// # Errors
    /// 읽기가 실패하면.
    pub fn count(&self) -> Result<usize, ProjectionError> {
        self.len_of(SYMBOL)
    }

    /// 파일 노드 하나.
    ///
    /// # Errors
    /// 읽기가 실패하거나 값을 풀지 못하면.
    pub fn file(&self, path: &RepoPath) -> Result<Option<FileRow>, ProjectionError> {
        let read = self.read()?;
        let Ok(t) = read.open_table(FILE) else { return Ok(None) };
        let Some(v) = t.get(path.as_str()).map_err(tx)? else { return Ok(None) };
        Ok(Some(
            postcard::from_bytes(&v.value()).map_err(|e| ProjectionError::Decode(e.to_string()))?,
        ))
    }

    /// 파일 노드 전부 — **경로 순.**
    ///
    /// # Errors
    /// 읽기가 실패하거나 값을 풀지 못하면.
    pub fn files(&self) -> Result<Vec<FileRow>, ProjectionError> {
        let read = self.read()?;
        let Ok(t) = read.open_table(FILE) else { return Ok(Vec::new()) };
        let mut out = Vec::new();
        for row in t.iter().map_err(tx)? {
            let (_, v) = row.map_err(tx)?;
            out.push(
                postcard::from_bytes(&v.value())
                    .map_err(|e| ProjectionError::Decode(e.to_string()))?,
            );
        }
        Ok(out)
    }

    /// 이 심볼이 **가리키는** 것들 — 정방향 인접. **정렬된다.**
    ///
    /// # Errors
    /// 읽기가 실패하면.
    pub fn callees(&self, id: SymbolId) -> Result<Vec<SymbolId>, ProjectionError> {
        self.adjacent(EDGE_OUT, id)
    }

    /// 이 심볼을 **가리키는** 것들 — 역방향. **`touch` 의 근간이다.**
    ///
    /// # Errors
    /// 읽기가 실패하면.
    pub fn callers(&self, id: SymbolId) -> Result<Vec<SymbolId>, ProjectionError> {
        self.adjacent(EDGE_IN, id)
    }

    /// 이 파일의 심볼 전부 — **경로·줄 순.**
    ///
    /// # Errors
    /// 읽기가 실패하거나 값을 풀지 못하면.
    pub fn symbols_of(&self, path: &RepoPath) -> Result<Vec<SymbolNode>, ProjectionError> {
        let read = self.read()?;
        let (Ok(by_file), Ok(by_id)) = (read.open_multimap_table(BY_FILE), read.open_table(SYMBOL))
        else {
            return Ok(Vec::new());
        };
        let mut out: Vec<SymbolNode> = Vec::new();
        for v in by_file.get(path.as_str()).map_err(tx)? {
            let v = v.map_err(tx)?;
            if let Some(raw) = by_id.get(v.value()).map_err(tx)? {
                out.push(
                    postcard::from_bytes(&raw.value())
                        .map_err(|e| ProjectionError::Decode(e.to_string()))?,
                );
            }
        }
        out.sort_by_key(|s| s.span.byte_start);
        Ok(out)
    }

    /// 노드와 엣지 전부 — **바깥 오라클이 읽는 창.**
    ///
    /// `scripts/f05-verify.py` 가 이것을 `sqlite3` 에 넣고 재귀 CTE 로 도달성을 계산해
    /// 우리 답과 댄다. **이 기능에서 유일하게 바깥에 있는 오라클이다**(R-18).
    ///
    /// # Errors
    /// 읽기가 실패하거나 값을 풀지 못하면.
    pub fn dump(&self) -> Result<GraphDump, ProjectionError> {
        let read = self.read()?;
        let mut nodes = Vec::new();
        if let Ok(t) = read.open_table(SYMBOL) {
            for row in t.iter().map_err(tx)? {
                let (_, v) = row.map_err(tx)?;
                nodes.push(
                    postcard::from_bytes(&v.value())
                        .map_err(|e| ProjectionError::Decode(e.to_string()))?,
                );
            }
        }
        let mut edges = Vec::new();
        if let Ok(t) = read.open_multimap_table(EDGE_OUT) {
            for row in t.iter().map_err(tx)? {
                let (k, vs) = row.map_err(tx)?;
                let Some(from) = symbol_id_of(k.value()) else { continue };
                for v in vs {
                    let v = v.map_err(tx)?;
                    if let Some(to) = symbol_id_of(v.value()) {
                        edges.push((from, to));
                    }
                }
            }
        }
        edges.sort();
        Ok((nodes, edges))
    }

    /// 질의 한 줄을 남긴다 — **append-only.** 순번을 돌려준다.
    ///
    /// # 덮어쓰지 않는다
    ///
    /// 순번은 이 스냅샷의 마지막 순번 + 1 이다. 같은 `(스냅샷, 순번)` 이 두 번 쓰이면
    /// 앞의 줄이 사라지고, 그러면 F17 이 세는 것이 질의 수가 아니게 된다.
    ///
    /// # Errors
    /// 쓰기가 실패하거나 값을 담지 못하면.
    pub fn log_query(
        &self,
        snapshot: &str,
        entry: &QueryLogEntry,
    ) -> Result<u64, ProjectionError> {
        let raw =
            postcard::to_allocvec(entry).map_err(|e| ProjectionError::Decode(e.to_string()))?;
        let write = self.write()?;
        let seq;
        {
            let mut t = write.open_table(QUERY_LOG).map_err(tx)?;
            seq = t
                .range((snapshot, 0u64)..=(snapshot, u64::MAX))
                .map_err(tx)?
                .next_back()
                .transpose()
                .map_err(tx)?
                .map_or(0, |(k, _)| k.value().1 + 1);
            t.insert((snapshot, seq), raw).map_err(tx)?;
        }
        write.commit().map_err(tx)?;
        Ok(seq)
    }

    /// 이 스냅샷에 쌓인 질의 줄 — **순번 순.**
    ///
    /// # Errors
    /// 읽기가 실패하거나 값을 풀지 못하면.
    pub fn query_log(&self, snapshot: &str) -> Result<Vec<QueryLogEntry>, ProjectionError> {
        let read = self.read()?;
        let Ok(t) = read.open_table(QUERY_LOG) else { return Ok(Vec::new()) };
        let mut out = Vec::new();
        for row in t.range((snapshot, 0u64)..=(snapshot, u64::MAX)).map_err(tx)? {
            let (_, v) = row.map_err(tx)?;
            out.push(
                postcard::from_bytes(&v.value())
                    .map_err(|e| ProjectionError::Decode(e.to_string()))?,
            );
        }
        Ok(out)
    }

    /// 파일이 그 이름으로 내보내는 심볼.
    ///
    /// # Errors
    /// 읽기가 실패하면.
    pub fn export(
        &self,
        path: &RepoPath,
        name: &str,
    ) -> Result<Option<SymbolId>, ProjectionError> {
        let read = self.read()?;
        let Ok(t) = read.open_table(EXPORTS) else { return Ok(None) };
        let Some(v) = t.get((path.as_str(), name)).map_err(tx)? else { return Ok(None) };
        Ok(symbol_id_of(v.value()))
    }

    /// 엣지 수 — **한 방향만 센다.** 두 방향의 수가 같아야 하고 그것이 `[f05.2.pass]` ① 이다.
    ///
    /// # Errors
    /// 읽기가 실패하면.
    pub fn edge_count(&self) -> Result<usize, ProjectionError> {
        self.multimap_len(EDGE_OUT)
    }

    /// 역방향 엣지 수. **정방향과 같아야 한다.**
    ///
    /// # Errors
    /// 읽기가 실패하면.
    pub fn reverse_edge_count(&self) -> Result<usize, ProjectionError> {
        self.multimap_len(EDGE_IN)
    }

    /// 내보내기 수.
    ///
    /// # Errors
    /// 읽기가 실패하면.
    pub fn export_count(&self) -> Result<usize, ProjectionError> {
        self.len_of(EXPORTS)
    }

    /// 파일 노드 수.
    ///
    /// # Errors
    /// 읽기가 실패하면.
    pub fn file_count(&self) -> Result<usize, ProjectionError> {
        self.len_of(FILE)
    }

    /// 이 투영이 어느 스냅샷에 대해 세워졌는가.
    ///
    /// **[`None`] 은 *"안 세워졌다"* 이지 *"아무 스냅샷"* 이 아니다.** 봉투의
    /// `built_for_this_snapshot` 이 지금까지 `true` 로 박혀 있었고, 그것은 관측이 아니라
    /// 기본값이었다.
    ///
    /// # Errors
    /// 읽기가 실패하면.
    pub fn built_for(&self) -> Result<Option<String>, ProjectionError> {
        let read = self.read()?;
        let Ok(t) = read.open_table(META) else { return Ok(None) };
        let Some(v) = t.get(META_BUILT_FOR).map_err(tx)? else { return Ok(None) };
        let raw = v.value();
        if raw.is_empty() { Ok(None) } else { Ok(Some(raw)) }
    }

    /// **지금 재구축 중인가** — 무대가 서 있으면 그렇다.
    ///
    /// DESIGN §12.7 격리 3번이 요구한 값이고, 봉투의 `projection.rebuild` 가 지금까지
    /// `NotBuilt{F05}` 였던 자리다. **관측 경로가 생겼으므로 이제 값이다.**
    ///
    /// # Errors
    /// 읽기가 실패하면.
    pub fn rebuilding(&self) -> Result<bool, ProjectionError> {
        Ok(self.table_names()?.iter().any(|n| n.ends_with(STAGE_SUFFIX)))
    }

    /// 이 투영에 실제로 서 있는 자리의 이름 전부 — **이름으로 세지 않고 실물에서 뜬다.**
    ///
    /// `[f05.2.pass]` ④ 가 이것을 쓴다. 목록을 코드에 박으면 새 자리가 생겨도 검사가
    /// 안 넓어지고, 그것이 F04 의 재구축 등가성 시험이 걸린 형태다.
    ///
    /// # Errors
    /// 읽기가 실패하면.
    pub fn table_names(&self) -> Result<Vec<String>, ProjectionError> {
        let read = self.read()?;
        let mut out: Vec<String> = Vec::new();
        for h in read.list_tables().map_err(tx)? {
            out.push(h.name().to_owned());
        }
        for h in read.list_multimap_tables().map_err(tx)? {
            out.push(h.name().to_owned());
        }
        out.sort();
        Ok(out)
    }

    /// 자리마다 행이 몇 개인가 — **이름순.** 전수 대조의 한쪽이다.
    ///
    /// # Errors
    /// 읽기가 실패하면.
    pub fn row_counts(&self) -> Result<BTreeMap<String, u64>, ProjectionError> {
        let read = self.read()?;
        let mut out = BTreeMap::new();
        for h in read.list_tables().map_err(tx)? {
            let name = h.name().to_owned();
            let t = read.open_untyped_table(h).map_err(tx)?;
            out.insert(name, t.len().map_err(tx)?);
        }
        for h in read.list_multimap_tables().map_err(tx)? {
            let name = h.name().to_owned();
            let t = read.open_untyped_multimap_table(h).map_err(tx)?;
            out.insert(name, t.len().map_err(tx)?);
        }
        Ok(out)
    }

    fn adjacent(
        &self,
        which: MultimapTableDefinition<&'static [u8], &'static [u8]>,
        id: SymbolId,
    ) -> Result<Vec<SymbolId>, ProjectionError> {
        let read = self.read()?;
        let Ok(t) = read.open_multimap_table(which) else { return Ok(Vec::new()) };
        let mut out = Vec::new();
        for v in t.get(id.as_bytes().as_slice()).map_err(tx)? {
            let v = v.map_err(tx)?;
            if let Some(s) = symbol_id_of(v.value()) {
                out.push(s);
            }
        }
        Ok(out)
    }

    fn len_of<K, V>(&self, which: TableDefinition<K, V>) -> Result<usize, ProjectionError>
    where
        K: redb::Key + 'static,
        V: redb::Value + 'static,
    {
        let read = self.read()?;
        let Ok(t) = read.open_table(which) else { return Ok(0) };
        let n: u64 = t.len().map_err(tx)?;
        Ok(usize::try_from(n).unwrap_or(usize::MAX))
    }

    fn multimap_len<K, V>(
        &self,
        which: MultimapTableDefinition<K, V>,
    ) -> Result<usize, ProjectionError>
    where
        K: redb::Key + 'static,
        V: redb::Key + 'static,
    {
        let read = self.read()?;
        let Ok(t) = read.open_multimap_table(which) else { return Ok(0) };
        let n: u64 = t.len().map_err(tx)?;
        Ok(usize::try_from(n).unwrap_or(usize::MAX))
    }
}

/// 노드와 엣지 전부 — [`Projection::dump`] 의 산출.
///
/// **이름을 붙여 가른다.** 벌거벗은 쌍이면 읽는 쪽이 `.0` 이 무엇인지 기억해야 하고,
/// 기억은 검사되지 않는다([`pal_core::Containment`] 와 같은 자리).
pub type GraphDump = (Vec<SymbolNode>, Vec<(SymbolId, SymbolId)>);

/// 32바이트가 아니면 그것은 좌표가 아니다. **조용히 0 으로 채우지 않는다.**
fn symbol_id_of(raw: &[u8]) -> Option<SymbolId> {
    let bytes: [u8; 32] = raw.try_into().ok()?;
    Some(SymbolId::from_bytes(bytes))
}

fn clear_stage(write: &WriteTransaction) -> Result<(), ProjectionError> {
    write.delete_table(SYMBOL_STAGE).map_err(tx)?;
    write.delete_multimap_table(BY_NAME_STAGE).map_err(tx)?;
    write.delete_table(FILE_STAGE).map_err(tx)?;
    write.delete_multimap_table(EDGE_OUT_STAGE).map_err(tx)?;
    write.delete_multimap_table(EDGE_IN_STAGE).map_err(tx)?;
    write.delete_multimap_table(BY_FILE_STAGE).map_err(tx)?;
    write.delete_table(EXPORTS_STAGE).map_err(tx)?;
    Ok(())
}

/// 자리를 만들어 둔다 — **파일이 0 개여도 교체가 성립해야 한다.**
fn open_stage(write: &WriteTransaction) -> Result<(), ProjectionError> {
    let _ = write.open_table(SYMBOL_STAGE).map_err(tx)?;
    let _ = write.open_multimap_table(BY_NAME_STAGE).map_err(tx)?;
    let _ = write.open_table(FILE_STAGE).map_err(tx)?;
    let _ = write.open_multimap_table(EDGE_OUT_STAGE).map_err(tx)?;
    let _ = write.open_multimap_table(EDGE_IN_STAGE).map_err(tx)?;
    let _ = write.open_multimap_table(BY_FILE_STAGE).map_err(tx)?;
    let _ = write.open_table(EXPORTS_STAGE).map_err(tx)?;
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// 반경 확장이 요구하는 이웃 — **F09**
//
// `pal-core::radius` 는 2층을 모르고 [`pal_core::Neighborhood`] 위에서 돈다. 여기가
// 그 트레잇의 유일한 실물 구현이고, 단위 시험의 손으로 만든 표와 **같은 함수를
// 지나간다**(`pal_core::expand`).
//
// # 오류를 빈 목록으로 접는다 — 그리고 그것이 안전한 방향인 이유
//
// 트레잇이 `Result` 를 안 진다. 읽기가 실패하면 이웃이 **비고**, 감시 집합은
// **대상 하나로 줄어든다** — 즉 *"덜 지켜본다"* 이지 *"틀린 것을 지켜본다"* 가 아니다.
// 그리고 그 축소는 조용하지 않다: 감시 집합 크기가 결박에 저장되고 `binding.status` 가
// 그것을 낸다. **`callers` 반경인데 크기가 1 이면 산출에서 보인다.**
impl pal_core::Neighborhood for Projection {
    fn callers_of(&self, s: SymbolId) -> Vec<SymbolId> {
        self.callers(s).unwrap_or_default()
    }

    fn symbols_in(&self, path: &RepoPath) -> Vec<SymbolId> {
        self.symbols_of(path).map(|v| v.into_iter().map(|n| n.id).collect()).unwrap_or_default()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 좌표 해소가 요구하는 조회 (F10 §3.2)
//
// [`pal_core::Neighborhood`] 와 **같은 규율이다** — 트레잇이 `Result` 를 안 지고,
// 읽기가 실패하면 후보가 **빈다.** 즉 *"덜 건다"* 이지 *"틀린 것을 건다"* 가 아니고,
// 그 축소는 조용하지 않다: 미결박은 `narrative.unbound` 가 **작업 목록으로** 낸다.
//
// ⚠ **후보 수를 자르지 않는다.** 자르면 *"여럿이라 못 좁혔다"* 의 규모가 사라지고,
// 사람은 **50 건짜리 제안과 2 건짜리 제안을 같은 화면으로 본다.** 후보의 수 자체가
// 산출이다(`TouchAnswer::Ambiguous` 와 같은 판단) — 조용한 절단 금지.
// ─────────────────────────────────────────────────────────────────────────────

fn 좌표로(n: SymbolNode) -> pal_core::NamedCoord {
    pal_core::NamedCoord { id: n.id, name: n.name, container: n.container, path: n.path }
}

impl pal_core::Coordinates for Projection {
    fn by_name(&self, name: &str) -> Vec<pal_core::NamedCoord> {
        self.resolve_name(name).map(|v| v.into_iter().map(좌표로).collect()).unwrap_or_default()
    }

    fn in_path(&self, path: &RepoPath) -> Vec<pal_core::NamedCoord> {
        self.symbols_of(path).map(|v| v.into_iter().map(좌표로).collect()).unwrap_or_default()
    }

}
