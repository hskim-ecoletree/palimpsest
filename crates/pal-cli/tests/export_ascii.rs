//! **병기가 기계 출력으로 새지 않는다** — `C1-a` 를 재는 자리.
//!
//! # 왜 시험인가 — 크레이트 경계가 이 자리는 안 막는다
//!
//! `crate::label` 은 `pal-core` 의 `parse` 열쇠와 `pal-query` 의 `watch_grades` 키에는
//! **구조로** 못 닿는다(다른 크레이트다). 그런데 `crate::export` 는 **같은 크레이트의
//! 형제 모듈**이라 아무것도 안 막는다 — 독립 리뷰 R1(발견 6)이 격리 사본에서
//! `quote(identity_name(s.identity))` 를 `quote(&crate::label::정체성_등급(…).병기())` 로
//! 바꿔 **컴파일 성공 · `cargo xtask check` 26/26 초록**을 실측했다.
//!
//! 그래서 그 자리는 여기서 잰다. **`pal export` 의 Cypher 산출에 한글이 한 글자도 없다.**
//!
//! # 음성 대조
//!
//! 판정을 순수 함수로 떼고, 병기가 얹힌 Cypher 조각을 일부러 먹여 발화를 관측한다.
//! 그것이 없으면 초록이 「한글이 없다」인지 「판정기가 아무것도 안 본다」인지 갈리지 않는다.

mod common;

/// 한글 음절이 하나라도 있으면 참. **순수 함수다.**
fn 한글이_있나(s: &str) -> bool {
    s.chars().any(|c| ('\u{AC00}'..='\u{D7A3}').contains(&c))
}

#[test]
fn cypher_산출에_한글이_없다() {
    let repo = common::저장소("export-ascii");
    // **결박 하나를 걸어 심볼 노드가 실제로 나가게 한다.** 산출이 비면 아래가 공짜다.
    common::pal(&repo, &["bind", "베타", "--note", "이 노트의 한국어는 Cypher 로 안 나간다"]);
    let out = common::pal(&repo, &["export", "--format", "cypher"]);

    // **하한** — 산출이 비면 아래가 공짜로 통과한다.
    assert!(
        out.contains("CREATE"),
        "Cypher 산출에 `CREATE` 가 없다 — 이 시험이 아무것도 안 잰다:\n{out}"
    );
    // ⚠ **심볼 이름은 대상이 아니다.** 픽스처의 `알파`·`베타` 는 소스의 식별자이고
    //   사용자 데이터다. 재는 것은 **우리 열거가 낸 속성 값** 셋뿐이다 —
    //   `kind`·`identity`·`grade` 가 각각 `SymbolKind`·`IdentityGrade`·`ExtractGrade`
    //   의 `name()` 을 지난다.
    let 잰_값 = 우리_열거가_낸_속성값(&out);
    assert!(
        잰_값.len() >= 3,
        "속성 값을 {}개만 봤다 — 이 시험이 아무것도 안 잰다:\n{out}",
        잰_값.len()
    );
    let 오염: Vec<&String> = 잰_값.iter().filter(|v| 한글이_있나(v)).collect();
    assert!(
        오염.is_empty(),
        "Cypher 속성 값에 한글이 있다 — 병기가 기계 출력으로 샜다: {오염:?}"
    );
}

/// `kind:`·`identity:`·`grade:` 뒤의 따옴표 값만 걷는다. **순수 함수다.**
fn 우리_열거가_낸_속성값(cypher: &str) -> Vec<String> {
    let mut o = Vec::new();
    for 키 in ["kind: \"", "identity: \"", "grade: \""] {
        let mut from = 0usize;
        while let Some(i) = cypher[from..].find(키) {
            let 시작 = from + i + 키.len();
            let Some(끝) = cypher[시작..].find('"') else { break };
            o.push(cypher[시작..시작 + 끝].to_owned());
            from = 시작 + 끝;
        }
    }
    o
}

/// **음성 대조** — 병기가 얹힌 줄을 먹이면 판정기가 잡는다.
#[test]
fn 병기가_얹히면_판정기가_잡는다() {
    let 오염 = r#"CREATE (:Symbol {id: "x", name: "알파", identity: "결박 불가(unavailable)"});"#;
    let 값 = 우리_열거가_낸_속성값(오염);
    assert_eq!(값, vec!["결박 불가(unavailable)".to_owned()], "속성 값을 잘못 걷었다: {값:?}");
    assert!(값.iter().any(|v| 한글이_있나(v)), "병기가 얹힌 값을 판정기가 못 잡았다");
    // 그리고 **심볼 이름은 안 걷는다** — 걷으면 사용자 데이터를 금지하는 시험이 된다.
    let 성한 = r#"CREATE (:Symbol {id: "x", name: "알파", identity: "unavailable"});"#;
    assert!(!우리_열거가_낸_속성값(성한).iter().any(|v| 한글이_있나(v)));
}
