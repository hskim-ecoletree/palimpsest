//! 버전 축. **하나로 합치면 캐시가 상시 전량 무효화된다** — stack §5.1.

use serde::Serialize;

/// 추출 결과를 무효화하는 축. 좌표에 실리는 유일한 버전이다.
///
/// 나머지 둘(`ProjectionVersion`·`PackFingerprint`)은 S0 에 없다 —
/// 2층도 팩도 아직 없기 때문이다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ExtractorVersion {
    /// tree-sitter 문법의 고정 커밋.
    pub grammar: &'static str,
    /// 추출기 코드 버전.
    pub extractor: &'static str,
}
