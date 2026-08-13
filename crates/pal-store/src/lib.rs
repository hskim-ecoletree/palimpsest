//! 파생 두 층 — 1층 콘텐츠 주소 캐시 + 2층 질의 투영.
//!
//! # 지우는 API 를 가진 유일한 크레이트다
//!
//! `pal cache prune` · `pal reindex` 가 여기 산다. 그래서 **이 크레이트는 의도 저장소에
//! 닿으면 안 된다** — 닿는 순간 캐시 폐기 경로가 사람의 승인 노동에 연결된다([R-21]).
//!
//! 규칙 둘을 `xtask` 가 기계로 검사한다(stack §4.1 · §4.3 단계 1):
//!
//! 1. `pal-store` 는 `pal-intent` 에 **의존하지 않는다** (나중에 읽기 전용으로만 허용)
//! 2. `pal-store` 소스에 의도를 지우는 호출이 **나타나지 않는다**
//!
//! # S1 시점의 상태
//!
//! **1층과 2층이 섰다.** [`BlobCache`] 는 콘텐츠 주소 파일 저장이고 키는 `(blob, 추출기 버전)`
//! 이다. 2층 인덱스는 여전히 비어 있다 — F05 의 것이다.
//!
//! **지우는 API 는 아직 없다.** `pal cache prune` 은 F04 이고, 그것이 생길 때 R-21 의
//! 검사가 실제로 하중을 진다. 지금은 경계만 서 있다.

#![forbid(unsafe_code)]

mod cache;
mod projection;

pub use cache::{BlobCache, CacheError, CacheKey, CacheStats, ExtractCache};
pub use projection::{Projection, ProjectionError};

/// 이 크레이트가 지키는 계약.
pub const CONTRACT: &str = "파생층은 의도 저장소를 지우지 않는다 — R-21";
