//! **TypeScript 지정자가 파일 경계를 넘는다** — 추출 → 1층 캐시 → 스티칭 → TS 모듈 해소
//! → `pal touch`.
//!
//! 회차 `2026-09-13-first-release-elsewhere` 의 `A1`·`A1-a`·`A2`·`A2-a`·`A4`. 합격선 정본은
//! 그 회차의 잠긴 의도다.
//!
//! # ⚠ 하한을 박는다
//!
//! 착수 시점에 남의 TS 저장소에서 파일 간 해소가 `0/11010` 이었고 까닭이 전부 「저장소 밖」
//! 이었다. 그 상태에서도 「엣지가 있다」는 파일 **안** 엣지로 참이므로, 이 파일의 단언은 전부
//! **다른 파일의 심볼**을 이름과 경로로 집는다.
//!
//! # 픽스처는 특정 저장소의 이름을 안 쓴다
//!
//! 해소 규칙은 TypeScript 일반이어야 한다 — 픽스처의 경로·별칭·설정은 이 파일이 지어낸 것이다.

mod common;

use common::{git, pal};
use pal_store::Projection;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// 파일 목록으로 git 저장소 하나를 세우고 커밋한다.
fn 저장소(tag: &str, files: &[(&str, &str)]) -> PathBuf {
    let root = std::env::temp_dir().join(format!("pal-ts-xfile-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    for (p, text) in files {
        let path = root.join(p);
        std::fs::create_dir_all(path.parent().expect("부모")).expect("디렉터리");
        std::fs::write(path, text).expect("쓰기");
    }
    git(&root, &["init", "-q", "."]);
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "user.email=t@example.com", "-c", "user.name=t", "commit", "-qm", "첫 커밋"]);
    root
}

fn 투영(repo: &Path) -> Projection {
    Projection::open(&repo.join(".palimpsest/index.redb")).expect("2층")
}

fn 씀(p: &Projection) -> pal_core::SymbolId {
    let mut found = p.resolve_name("씀").expect("이름");
    assert_eq!(found.len(), 1, "픽스처에 `씀` 이 하나여야 한다");
    found.remove(0).id
}

/// `씀` 이 가리키는 것 중 **다른 파일의** 심볼 — `(이름, 경로)`.
fn 파일_간_대상(p: &Projection) -> Vec<(String, String)> {
    let from = 씀(p);
    let mine = p.symbol(from).expect("심볼").expect("있다").path;
    let mut out: Vec<(String, String)> = p
        .callees(from)
        .expect("정방향")
        .into_iter()
        .filter_map(|id| p.symbol(id).ok().flatten())
        .filter(|s| s.path != mine)
        .map(|s| (s.name.clone(), s.path.as_str().to_owned()))
        .collect();
    out.sort();
    out
}

/// `씀` 에서 못 푼 참조 — 이름 → 까닭.
fn 못푼_까닭(p: &Projection) -> BTreeMap<String, String> {
    let from = 씀(p);
    p.unresolved_refs()
        .expect("못 푼 참조")
        .expect("파일 간 해소 패스를 지났다")
        .into_iter()
        .filter(|u| u.site == from)
        .map(|u| (u.name.clone(), u.reason.as_str().to_owned()))
        .collect()
}

fn 대상(pairs: &[(&str, &str)]) -> Vec<(String, String)> {
    let mut v: Vec<(String, String)> = pairs.iter().map(|(a, b)| ((*a).to_owned(), (*b).to_owned())).collect();
    v.sort();
    v
}

#[test]
fn a1_지정자_여섯_꼴이_다른_파일의_심볼로_가는_엣지가_된다() {
    let root = 저장소(
        "a1",
        &[
            ("tsconfig.json", r#"{"compilerOptions":{"moduleResolution":"bundler","baseUrl":".","paths":{"@app/*":["./src/*"]}}}"#),
            ("src/a.ts", "export function fromA() { return 1; }\n"),
            ("src/b/c.ts", "export function fromC() { return 2; }\n"),
            ("src/x.ts", "export function fromX() { return 3; }\n"),
            ("src/y.ts", "export function fromY() { return 4; }\n"),
            ("src/dir/index.ts", "export function fromDir() { return 5; }\n"),
            ("src/alias/target.ts", "export function fromAlias() { return 6; }\n"),
            ("src/user/sib.ts", "export function fromSib() { return 7; }\n"),
            (
                "src/user/use.ts",
                "import { fromSib } from './sib';\n\
                 import { fromA } from '../a';\n\
                 import { fromC } from '../b/c';\n\
                 import { fromX } from '../x.js';\n\
                 import { fromY } from '../y.ts';\n\
                 import { fromDir } from '../dir';\n\
                 import { fromAlias } from '@app/alias/target';\n\
                 import { readFileSync } from 'node:fs';\n\
                 import { z } from 'zod';\n\
                 import { gone } from './missing';\n\
                 export function 씀() {\n\
                 \x20 return fromSib() + fromA() + fromC() + fromX() + fromY() + fromDir() + fromAlias()\n\
                 \x20   + readFileSync.length + z.length + gone();\n\
                 }\n",
            ),
        ],
    );
    pal(&root, &["touch", "씀"]);
    let p = 투영(&root);
    assert_eq!(
        파일_간_대상(&p),
        대상(&[
            ("fromA", "src/a.ts"),
            ("fromAlias", "src/alias/target.ts"),
            ("fromC", "src/b/c.ts"),
            ("fromDir", "src/dir/index.ts"),
            ("fromSib", "src/user/sib.ts"),
            ("fromX", "src/x.ts"),
            ("fromY", "src/y.ts"),
        ]),
    );

    // `A1-a` — 음성 대조. **아무 지정자나 무언가에 잇는 해소기는 여기서 빨개진다.**
    let 까닭 = 못푼_까닭(&p);
    assert_eq!(까닭.get("readFileSync").map(String::as_str), Some("outside_repo"));
    assert_eq!(까닭.get("z").map(String::as_str), Some("bare_specifier"), "맨 지정자를 「저장소 밖」으로 적지 않는다");
    assert_eq!(까닭.get("gone").map(String::as_str), Some("no_target_file"));
    let _ = std::fs::remove_dir_all(&root);
}

/// `A2` 의 한 칸 — 설정 파일들과 쓰는 파일 하나를 두고 별칭 임포트의 대상 경로를 잰다.
fn 별칭이_간다(tag: &str, files: &[(&str, &str)], 기대: &[(&str, &str)]) {
    let root = 저장소(tag, files);
    pal(&root, &["touch", "씀"]);
    assert_eq!(파일_간_대상(&투영(&root)), 대상(기대), "픽스처 {tag}");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a2_1_주석과_꼬리_쉼표가_있는_tsconfig_를_읽는다() {
    별칭이_간다(
        "a2-1",
        &[
            ("tsconfig.json", "{\n  // 주석\n  \"compilerOptions\": {\n    /* 블록 */ \"baseUrl\": \".\",\n    \"paths\": { \"@app/*\": [\"src/*\",], },\n  },\n}\n"),
            ("src/m.ts", "export function 모듈() { return 1; }\n"),
            ("use.ts", "import { 모듈 } from '@app/m';\nexport function 씀() { return 모듈(); }\n"),
        ],
        &[("모듈", "src/m.ts")],
    );
}

#[test]
fn a2_2_저장소_안_extends_사슬로_온_paths_를_읽는다() {
    별칭이_간다(
        "a2-2",
        &[
            ("tsconfig.json", r#"{"extends":"./configs/mid.json"}"#),
            ("configs/mid.json", r#"{"extends":"./base"}"#),
            ("configs/base.json", r#"{"compilerOptions":{"baseUrl":"..","paths":{"@lib/*":["lib/*"]}}}"#),
            ("lib/util.ts", "export function 유틸() { return 1; }\n"),
            ("use.ts", "import { 유틸 } from '@lib/util';\nexport function 씀() { return 유틸(); }\n"),
        ],
        &[("유틸", "lib/util.ts")],
    );
}

#[test]
fn a2_3_baseurl_없는_paths_는_설정_파일의_자리를_기준으로_편다() {
    별칭이_간다(
        "a2-3",
        &[
            ("tsconfig.json", r##"{"compilerOptions":{"paths":{"#src/*":["./src/*"]}}}"##),
            ("src/m.ts", "export function 모듈() { return 1; }\n"),
            ("other/use.ts", "import { 모듈 } from '#src/m';\nexport function 씀() { return 모듈(); }\n"),
        ],
        &[("모듈", "src/m.ts")],
    );
}

#[test]
fn a2_4_하위_디렉터리의_더_가까운_tsconfig_가_이긴다() {
    별칭이_간다(
        "a2-4",
        &[
            ("tsconfig.json", r#"{"compilerOptions":{"paths":{"@/*":["./outer/*"]}}}"#),
            ("pkg/tsconfig.json", r#"{"compilerOptions":{"paths":{"@/*":["./inner/*"]}}}"#),
            ("outer/a.ts", "export function 둘() { return 1; }\n"),
            ("pkg/inner/a.ts", "export function 둘() { return 2; }\n"),
            ("pkg/src/use.ts", "import { 둘 } from '@/a';\nexport function 씀() { return 둘(); }\n"),
        ],
        &[("둘", "pkg/inner/a.ts")],
    );
}

#[test]
fn a2_5_tsconfig_는_답이_선_트리에서_읽는다() {
    let root = 저장소(
        "a2-5",
        &[
            ("tsconfig.json", r#"{"compilerOptions":{"paths":{"@/*":["./one/*"]}}}"#),
            ("one/m.ts", "export function 모듈() { return 1; }\n"),
            ("two/m.ts", "export function 모듈() { return 2; }\n"),
            ("use.ts", "import { 모듈 } from '@/m';\nexport function 씀() { return 모듈(); }\n"),
        ],
    );
    // 커밋 안 된 편집 — 별칭을 `two` 로 돌린다.
    std::fs::write(root.join("tsconfig.json"), r#"{"compilerOptions":{"paths":{"@/*":["./two/*"]}}}"#).expect("쓰기");

    pal(&root, &["touch", "씀"]);
    assert_eq!(파일_간_대상(&투영(&root)), 대상(&[("모듈", "two/m.ts")]), "기본은 워킹트리다");

    pal(&root, &["touch", "씀", "--at", "HEAD"]);
    assert_eq!(파일_간_대상(&투영(&root)), 대상(&[("모듈", "one/m.ts")]), "--at HEAD 는 커밋된 설정이다");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a2_6_상속된_paths_는_그것을_정의한_설정의_자리를_기준으로_편다() {
    별칭이_간다(
        "a2-6",
        &[
            ("configs/base.json", r#"{"compilerOptions":{"paths":{"@x/*":["./x/*"]}}}"#),
            ("app/tsconfig.json", r#"{"extends":"../configs/base.json"}"#),
            ("configs/x/m.ts", "export function 모듈() { return 1; }\n"),
            // 미끼 — 자식 설정의 자리로 풀면 여기로 간다.
            ("app/x/m.ts", "export function 모듈() { return 2; }\n"),
            ("app/use.ts", "import { 모듈 } from '@x/m';\nexport function 씀() { return 모듈(); }\n"),
        ],
        &[("모듈", "configs/x/m.ts")],
    );
}

#[test]
fn a2a_패키지_extends_에서만_오는_별칭은_저장소_밖이_아니라_못_읽음이다() {
    let root = 저장소(
        "a2a",
        &[
            ("tsconfig.json", r#"{"extends":"@tsconfig/strictest/tsconfig.json"}"#),
            ("src/m.ts", "export function 모듈() { return 1; }\n"),
            ("use.ts", "import { 모듈 } from '@/m';\nexport function 씀() { return 모듈(); }\n"),
        ],
    );
    pal(&root, &["touch", "씀"]);
    let 까닭 = 못푼_까닭(&투영(&root));
    assert_eq!(까닭.get("모듈").map(String::as_str), Some("config_incomplete"));
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a4_재수출_배럴로_펴지는_임포트는_재수출을_지나는_이름이다() {
    let root = 저장소(
        "a4",
        &[
            ("tsconfig.json", r#"{"compilerOptions":{"moduleResolution":"bundler"}}"#),
            // 선언 0 · 재수출만 있는 배럴.
            ("src/hosts/index.ts", "export { hostA } from './a';\n"),
            ("src/hosts/a.ts", "export function hostA() { return 1; }\n"),
            // 선언과 `export *` 가 섞인 배럴.
            ("src/mixed/index.ts", "export function own() { return 2; }\nexport * from './b';\n"),
            ("src/mixed/b.ts", "export function viaStar() { return 3; }\n"),
            // 대조 — 재수출이 없고 그 이름의 선언도 없다.
            ("src/plain.ts", "export function other() { return 4; }\n"),
            (
                "src/use.ts",
                "import { hostA } from './hosts';\n\
                 import { viaStar, own } from './mixed';\n\
                 import { nope } from './plain';\n\
                 export function 씀() { return hostA() + viaStar() + own() + nope(); }\n",
            ),
        ],
    );
    pal(&root, &["touch", "씀"]);
    let p = 투영(&root);
    let 까닭 = 못푼_까닭(&p);
    assert_eq!(까닭.get("hostA").map(String::as_str), Some("through_reexport"), "선언 0 인 배럴");
    assert_eq!(까닭.get("viaStar").map(String::as_str), Some("through_reexport"), "`export *` 가 섞인 배럴");
    assert_eq!(까닭.get("nope").map(String::as_str), Some("no_symbol"), "재수출이 없는 파일은 대조군이다");
    assert_eq!(파일_간_대상(&p), 대상(&[("own", "src/mixed/index.ts")]), "배럴의 자기 선언은 엣지다");
    let _ = std::fs::remove_dir_all(&root);
}

/// **화면의 설명문이 한 언어의 뜻을 싣지 않는다** (회차 계획 ㈀ 「문구는 언어 중립이다」).
///
/// 이관표와 참조 엣지 단서는 언어를 안 가리고 같은 문장을 찍는다. 그 문장이 Rust 의 이름
/// (크레이트 뿌리 · 구조체 리터럴 · 연관 상수)이나 TypeScript 의 이름(`tsconfig` · `classic`)을
/// 실으면 다른 언어의 사용자는 자기 저장소에 없는 것을 읽는다. 열쇠(`no_symbol_at_crate_root`
/// 따위)는 회계 이름이라 그대로 둔다 — 여기서 재는 것은 뜻을 푼 문장이다.
#[test]
fn 화면의_설명문은_한_언어의_이름을_싣지_않는다() {
    let root = 저장소(
        "neutral",
        &[
            ("src/a.ts", "export function fromA() { return 1; }\n"),
            (
                "src/use.ts",
                "import { fromA } from './a';\nimport { gone } from './missing';\n\
                 export function 씀() { return fromA() + gone(); }\n",
            ),
        ],
    );
    let 화면 = pal(&root, &["touch", "씀"]);
    assert!(화면.contains("못 선 까닭의 성격"), "이관표가 안 찍혀 이 시험이 아무것도 안 잰다:\n{화면}");
    let 설명: String = 화면
        .lines()
        .filter(|l| l.contains("※") || l.contains("` →"))
        .map(|l| l.split_once("` →").map_or(l, |(_, 뜻)| 뜻))
        .collect::<Vec<_>>()
        .join("\n");
    for 이름 in ["크레이트", "구조체", "연관 상수", "열거형 변형", "tsconfig", "classic"] {
        assert!(!설명.contains(이름), "설명문이 한 언어의 이름 {이름} 을 싣는다:\n{설명}");
    }
    let _ = std::fs::remove_dir_all(&root);
}
