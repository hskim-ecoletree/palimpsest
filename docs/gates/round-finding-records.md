# 게이트 — 발견 레코드의 자리와 그것을 재는 검사

> 회차 `2026-08-19-finding-records` · 착수 `47a6770` · 이슈 [#71] · [#72]
> 절 이름은 규약 §9 가 정한 넷으로 고정한다.

## 합격선

**측정 전에 등록했다** — 착수 커밋 `0af61cb`. 전문은
[`intent.md`](../../.palimpsest/rounds/2026-08-19-finding-records/intent.md) 의 `## 완수 조건`.

**조건 46 개**를 열둘로 묶었다: C1 스키마 · C2 쓰는 자 · C3 재는 자 · C4 계기판 ·
C5 설치 표면 · C6 에이전트 정의 · C7 이 회차의 발견 · C8 규약 · C9 「당기는가」 ·
C10 종료 여섯 · C11 수용 사유 · C12 (나) 분할.

**RED — 착수 시점에 실제로 관측했다**
([`red/red-observed.txt`](../../.palimpsest/rounds/2026-08-19-finding-records/red/red-observed.txt)):

    계기판이 내는 칸                      여섯 — 「원 의도 비율」·「발견 유효성」이 없다
    .palimpsest/rounds 아래 .jsonl        0 건
    「기각」을 필드로 가진 기계 판독 파일   0 건
    retro/02-classification.tsv           110 줄 · 어느 검사의 모집단에도 없다

**음성 대조 — 「이 검사가 고장이면 이렇게 드러난다」** (전문:
[`effect/negative-control.txt`](../../.palimpsest/rounds/2026-08-19-finding-records/effect/negative-control.txt))

격리 사본에서 **그 사본의 `xtask` 를 빌드해** 일곱을 심었고 **일곱이 전부 발화**했다 —
enum 밖 값 · 필수 필드 누락 · **빈 모집단** · 없는 경로 · **합계 검산 어긋남** ·
tsv 열 수 어긋남 · 머리 줄 없음.

⚠ **두 번 실패한 뒤에 섰다.** 첫 두 번은 일곱이 **전부 「20/20 통과」**로 보였다 —
검사가 옳아서가 아니라 **대조가 안 선 것**이다. 까닭 둘: `git clone --depth 1` 이
커밋된 것만 가져왔고, 원 저장소의 `xtask` 는 `repo_root()` 가
`env!("CARGO_MANIFEST_DIR")` 로 **컴파일 시점 경로**를 박는다.

**퇴로 (등록했고 쓰지 않았다)**: 계기판 칸 둘을 내리고 자리+검사만 세운다.

**상한**: 인터뷰 4 · 사전부검 3 · 독립 리뷰 5.

## 판정

⏳ 독립 리뷰가 닫힌 뒤에 적는다.

## 효과

**테스트도 CI 도 아닌 것이 돌린 출력**이다. 전문은
[`effect/effect.md`](../../.palimpsest/rounds/2026-08-19-finding-records/effect/effect.md) ·
[`effect/ditto-control.md`](../../.palimpsest/rounds/2026-08-19-finding-records/effect/ditto-control.md).

**물음**: *에이전트가 낸 발견 중 몇 %가 헛것인가?*

앞 회차가 축 1 로 재려다 **참 109 · 거짓 0** 으로 반증된 물음이다. 축이 고장난 것이
아니라 **모집단이 고장난 것**이었다 — 커밋에 남는 것은 채택된 발견뿐이다.

| | 총 | 참 | 거짓 | 헛것 |
|---|--:|--:|--:|--:|
| **palimpsest 회차 D** | 90 | 68 | 22 | **24%** |
| **ditto (대조군)** | 83 | 50 | 33 | **39%** |

★ **두 하네스를 같은 축에서 처음으로 비교했다.** 기각된 발견이 남는 자리가 생기자
[#72] 가 *"이 저장소에서는 그 수를 못 센다"* 고 적은 것이 닫혔다.

⚠ **비교의 한계**: 두 수는 **같은 정의가 아니다.** ditto 의 `admissible=false` 는
심판이 각하한 것이고, 이쪽의 `유효성=거짓` 은 **발견자가 스스로 물린 것**이다.
같은 축에 놓았지만 **같은 자로 잰 것은 아니다.**

## 범위 밖

이 게이트가 답하지 않기로 한 물음. 착수 때 정했고 회차 중에 새로 안 생겼다.

- **108 건(`retro/02-classification.tsv`) 소급 이전** — **재는 것과 옮기는 것은 다르다.**
  그 파일은 이제 검사 모집단에 들지만 열이 새 스키마로 바뀌지는 않는다
- **#66** Rust·Markdown 추출기 · **#69** 문서 간 결박 · **#68** sunset 처분의 실행
- 하네스의 완성 장면·새 지형 — 병렬 회차 B 가 진다
- **`pal` CLI 에 서브커맨드를 더하는 것** — 소유자가 표면을 스킬 `bin/` 으로 골랐다

[#71]: https://github.com/hskim-ecoletree/palimpsest/issues/71
[#72]: https://github.com/hskim-ecoletree/palimpsest/issues/72
