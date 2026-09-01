//! `verification.log` schema 1 parser와 상태 전이 검증.

use std::collections::BTreeMap;
use std::path::Path;

use serde::Deserialize;
use thiserror::Error;

use pal_core::{
    ROUND_VERIFICATION_FILE_MAX_BYTES, ROUND_VERIFICATION_LINE_MAX_BYTES,
    ROUND_VERIFICATION_STRING_MAX_BYTES,
};

const HEX_LEN: usize = 64;
const DOMAIN: &[u8] = b"pal.round.oracle.v1\0";
const DOMAIN_V2: &[u8] = b"pal.round.oracle.v2\0";

#[derive(Clone, Debug)]
pub struct Oracle {
    pub digest: String,
    pub check: String,
    pub literal: String,
    pub cwd: String,
    pub negative_for: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Evidence {
    pub oracle_digest: String,
    pub exit: i32,
    pub matched: bool,
    pub projected_digest: Option<String>,
}

#[derive(Clone, Debug)]
pub struct EvidenceRef {
    pub path: String,
    pub digest: String,
}

#[derive(Clone, Debug)]
pub struct Judgment {
    pub met: bool,
    pub thesis: EvidenceRef,
    pub antithesis: EvidenceRef,
    pub synthesis: EvidenceRef,
}

#[derive(Clone, Debug)]
pub struct Checkpoint {
    pub projected_digest: String,
    pub aggregate_digest: String,
    pub finalization_seal: String,
}

#[derive(Clone, Debug, Default)]
pub struct ConditionLedger {
    pub oracle: Option<Oracle>,
    pub evidence: Option<Evidence>,
    pub judgment: Option<Judgment>,
    pub had_evidence_before_current_oracle: bool,
}

#[derive(Clone, Debug)]
pub struct VerificationLedger {
    pub schema_version: u32,
    pub conditions: BTreeMap<String, ConditionLedger>,
    pub checkpoint: Option<Checkpoint>,
}

#[derive(Debug, Error)]
pub enum LedgerError {
    #[error("원장을 읽지 못했다: {0}")]
    Io(#[from] std::io::Error),
    #[error("verification schema 오류: {0}")]
    Schema(String),
    #[error("verification 상태 전이 오류: {0}")]
    Transition(String),
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", deny_unknown_fields)]
enum Event {
    #[serde(rename = "schema")]
    Schema { version: u32, round: String },
    #[serde(rename = "oracle")]
    Oracle {
        id: String,
        mode: Mode,
        check: String,
        expect: Expect,
        cwd: String,
        #[serde(default)]
        negative_for: Option<String>,
    },
    #[serde(rename = "evidence")]
    Evidence {
        id: String,
        oracle_digest: String,
        exit: i32,
        matched: bool,
        output_digest: String,
        #[serde(rename = "output_bytes")]
        _output_bytes: u64,
        #[serde(default)]
        projected_digest: Option<String>,
    },
    #[serde(rename = "judgment")]
    Judgment {
        id: String,
        verdict: Verdict,
        thesis: EventRef,
        antithesis: EventRef,
        synthesis: EventRef,
    },
    #[serde(rename = "checkpoint")]
    Checkpoint {
        projected_digest: String,
        aggregate_digest: String,
        finalization_seal: String,
    },
}

#[derive(Debug, Deserialize)]
enum Mode {
    #[serde(rename = "command")]
    Command,
}

#[derive(Debug, Deserialize)]
enum Verdict {
    #[serde(rename = "met")]
    Met,
    #[serde(rename = "unmet")]
    Unmet,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct EventRef {
    path: String,
    digest: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Expect {
    literal: String,
}

pub fn read(path: &Path, slug: &str) -> Result<VerificationLedger, LedgerError> {
    let metadata = std::fs::metadata(path)?;
    if metadata.len() > ROUND_VERIFICATION_FILE_MAX_BYTES {
        return Err(schema("파일이 8 MiB 상한을 넘었다"));
    }
    let bytes = std::fs::read(path)?;
    if bytes.is_empty() || !bytes.ends_with(b"\n") {
        return Err(schema("빈 파일 또는 trailing partial line이다"));
    }
    let text = std::str::from_utf8(&bytes).map_err(|e| schema(format!("UTF-8이 아니다: {e}")))?;
    let mut events = Vec::new();
    for (index, raw) in text.split_terminator('\n').enumerate() {
        let line = raw.strip_suffix('\r').unwrap_or(raw);
        if line.is_empty() {
            return Err(schema(format!("{}행이 비었다", index + 1)));
        }
        if line.len() > ROUND_VERIFICATION_LINE_MAX_BYTES {
            return Err(schema(format!("{}행이 64 KiB 상한을 넘었다", index + 1)));
        }
        let event: Event =
            serde_json::from_str(line).map_err(|e| schema(format!("{}행 JSON: {e}", index + 1)))?;
        events.push(event);
    }

    let mut schema_seen = false;
    let mut schema_version = 0;
    let mut conditions: BTreeMap<String, ConditionLedger> = BTreeMap::new();
    let mut checkpoint = None;
    for (index, event) in events.into_iter().enumerate() {
        match event {
            Event::Schema { version, round } => {
                if index != 0 || schema_seen {
                    return Err(schema("schema 행은 첫 행에 정확히 하나여야 한다"));
                }
                schema_seen = true;
                if !matches!(version, 1 | 2 | 3) {
                    return Err(schema(format!("알 수 없는 schema version {version}")));
                }
                schema_version = version;
                validate_string("round", &round, false)?;
                if !valid_slug(&round) || round != slug {
                    return Err(schema("schema round가 디렉터리 slug와 다르다"));
                }
            }
            Event::Oracle {
                id,
                mode: Mode::Command,
                check,
                expect,
                cwd,
                negative_for,
            } => {
                if !schema_seen {
                    return Err(schema("oracle보다 schema가 먼저 와야 한다"));
                }
                validate_id(&id)?;
                validate_string("check", &check, true)?;
                validate_string("expect.literal", &expect.literal, true)?;
                validate_string("cwd", &cwd, true)?;
                if !valid_cwd(&cwd) {
                    return Err(schema("cwd는 정규화된 저장소 상대 경로여야 한다"));
                }
                if schema_version == 1 && negative_for.is_some() {
                    return Err(schema("schema 1 oracle에는 negative_for가 없다"));
                }
                if let Some(base) = &negative_for {
                    validate_id(base)?;
                    if base == &id {
                        return Err(schema("negative control은 자신을 가리킬 수 없다"));
                    }
                }
                let digest = if schema_version == 1 {
                    oracle_digest("command", &check, &expect.literal, &cwd)
                } else {
                    oracle_digest_v2(
                        "command",
                        &check,
                        &expect.literal,
                        &cwd,
                        negative_for.as_deref(),
                    )
                };
                let state = conditions.entry(id).or_default();
                state.had_evidence_before_current_oracle |= state.evidence.is_some();
                if state.judgment.is_some() {
                    return Err(schema("condition은 command와 dialectic을 함께 가질 수 없다"));
                }
                state.oracle = Some(Oracle {
                    digest,
                    check,
                    literal: expect.literal,
                    cwd,
                    negative_for,
                });
                state.evidence = None;
            }
            Event::Evidence {
                id,
                oracle_digest,
                exit,
                matched,
                output_digest,
                _output_bytes: _,
                projected_digest,
            } => {
                if !schema_seen {
                    return Err(schema("evidence보다 schema가 먼저 와야 한다"));
                }
                validate_id(&id)?;
                validate_digest("oracle_digest", &oracle_digest)?;
                validate_digest("output_digest", &output_digest)?;
                match (schema_version, &projected_digest) {
                    (1, None) => {}
                    (1, Some(_)) => {
                        return Err(schema("schema 1 evidence에는 projected_digest가 없다"));
                    }
                    (2 | 3, Some(digest)) => validate_digest("projected_digest", digest)?,
                    (2 | 3, None) => {
                        return Err(schema("schema 2/3 evidence에는 projected_digest가 필요하다"));
                    }
                    _ => unreachable!("schema version was validated"),
                }
                let Some(state) = conditions.get_mut(&id) else {
                    return Err(LedgerError::Transition(format!(
                        "oracle 없는 evidence `{id}`"
                    )));
                };
                state.evidence = Some(Evidence {
                    oracle_digest,
                    exit,
                    matched,
                    projected_digest,
                });
            }
            Event::Judgment {
                id,
                verdict,
                thesis,
                antithesis,
                synthesis,
            } => {
                if schema_version != 3 {
                    return Err(schema("judgment는 schema 3에서만 쓴다"));
                }
                validate_id(&id)?;
                let thesis = validate_ref("thesis", thesis)?;
                let antithesis = validate_ref("antithesis", antithesis)?;
                let synthesis = validate_ref("synthesis", synthesis)?;
                if thesis.path == antithesis.path
                    || thesis.path == synthesis.path
                    || antithesis.path == synthesis.path
                {
                    return Err(schema("정·반·합 evidence 경로는 서로 달라야 한다"));
                }
                let state = conditions.entry(id).or_default();
                if state.oracle.is_some() {
                    return Err(schema("condition은 command와 dialectic을 함께 가질 수 없다"));
                }
                state.judgment = Some(Judgment {
                    met: matches!(verdict, Verdict::Met),
                    thesis,
                    antithesis,
                    synthesis,
                });
            }
            Event::Checkpoint {
                projected_digest,
                aggregate_digest,
                finalization_seal,
            } => {
                if schema_version != 3 {
                    return Err(schema("checkpoint는 schema 3에서만 쓴다"));
                }
                validate_digest("projected_digest", &projected_digest)?;
                validate_digest("aggregate_digest", &aggregate_digest)?;
                validate_digest("finalization_seal", &finalization_seal)?;
                checkpoint = Some(Checkpoint {
                    projected_digest,
                    aggregate_digest,
                    finalization_seal,
                });
            }
        }
    }
    if !schema_seen {
        return Err(schema("schema 행이 없다"));
    }
    if schema_version >= 2 {
        for (id, state) in &conditions {
            let Some(control) = state
                .oracle
                .as_ref()
                .and_then(|oracle| oracle.negative_for.as_ref())
            else {
                continue;
            };
            let Some(base) = conditions
                .get(control)
                .and_then(|state| state.oracle.as_ref())
            else {
                return Err(schema(format!(
                    "negative control `{id}`의 대상 `{control}` oracle이 없다"
                )));
            };
            let oracle = state.oracle.as_ref().expect("checked above");
            if base.negative_for.is_some() {
                return Err(schema("negative control chain은 허용하지 않는다"));
            }
            if base.digest == oracle.digest {
                return Err(schema("negative control은 주 oracle과 같은 실행일 수 없다"));
            }
        }
    }
    Ok(VerificationLedger {
        schema_version,
        conditions,
        checkpoint,
    })
}

/// 새 JSON 행을 더한 원장이 reader의 파일·행 상한 안에 그대로 되읽힐 수 있는가.
#[must_use]
pub fn append_fits(current_bytes: usize, line_bytes: usize) -> bool {
    line_bytes <= ROUND_VERIFICATION_LINE_MAX_BYTES
        && line_bytes
            .checked_add(1)
            .and_then(|added| current_bytes.checked_add(added))
            .is_some_and(|total| total <= ROUND_VERIFICATION_FILE_MAX_BYTES as usize)
}

pub fn oracle_digest(mode: &str, check: &str, literal: &str, cwd: &str) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(DOMAIN);
    for value in [mode, check, "literal", literal, cwd] {
        let bytes = value.as_bytes();
        hasher.update(&(bytes.len() as u64).to_le_bytes());
        hasher.update(bytes);
    }
    hasher.finalize().to_hex().to_string()
}

fn oracle_digest_v2(
    mode: &str,
    check: &str,
    literal: &str,
    cwd: &str,
    negative_for: Option<&str>,
) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(DOMAIN_V2);
    for value in [
        mode,
        check,
        "literal",
        literal,
        cwd,
        negative_for.unwrap_or(""),
    ] {
        let bytes = value.as_bytes();
        hasher.update(&(bytes.len() as u64).to_le_bytes());
        hasher.update(bytes);
    }
    hasher.finalize().to_hex().to_string()
}

fn schema(message: impl Into<String>) -> LedgerError {
    LedgerError::Schema(message.into())
}

fn validate_id(id: &str) -> Result<(), LedgerError> {
    if pal_intent::round_condition::ConditionId::parse(id).is_none() {
        return Err(schema(format!("유효하지 않은 condition id `{id}`")));
    }
    Ok(())
}

fn validate_string(name: &str, value: &str, nonempty: bool) -> Result<(), LedgerError> {
    if nonempty && value.is_empty() {
        return Err(schema(format!("{name}은 비어 있을 수 없다")));
    }
    if value.len() > ROUND_VERIFICATION_STRING_MAX_BYTES {
        return Err(schema(format!("{name}이 32 KiB 상한을 넘었다")));
    }
    Ok(())
}

fn validate_digest(name: &str, value: &str) -> Result<(), LedgerError> {
    if value.len() != HEX_LEN
        || !value
            .bytes()
            .all(|b| b.is_ascii_digit() || matches!(b, b'a'..=b'f'))
    {
        return Err(schema(format!("{name}는 소문자 64자리 hex여야 한다")));
    }
    Ok(())
}

fn validate_ref(name: &str, value: EventRef) -> Result<EvidenceRef, LedgerError> {
    validate_string(&format!("{name}.path"), &value.path, true)?;
    if !valid_cwd(&value.path) || value.path == "." {
        return Err(schema(format!(
            "{name}.path는 정규화된 저장소 상대 파일 경로여야 한다"
        )));
    }
    validate_digest(&format!("{name}.digest"), &value.digest)?;
    Ok(EvidenceRef {
        path: value.path,
        digest: value.digest,
    })
}

fn valid_slug(value: &str) -> bool {
    !value.is_empty()
        && (value.as_bytes()[0].is_ascii_lowercase() || value.as_bytes()[0].is_ascii_digit())
        && value
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

fn valid_cwd(value: &str) -> bool {
    if value == "." {
        return true;
    }
    !value.is_empty()
        && !value.starts_with('/')
        && !value.ends_with('/')
        && !value.contains('\\')
        && !value.contains(':')
        && value
            .split('/')
            .all(|part| !part.is_empty() && part != "." && part != "..")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp(tag: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!("pal-ledger-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_file(&path);
        path
    }

    fn write_read(tag: &str, body: &str) -> Result<VerificationLedger, LedgerError> {
        let path = temp(tag);
        std::fs::write(&path, body).expect("fixture");
        read(&path, "fixture-round")
    }

    #[test]
    fn locked_digest_vector() {
        assert_eq!(
            oracle_digest("command", "cargo test -q", "ROUND_OK", "."),
            "4cf3cb926ab8249a040632d0c1e694509ab40eee2eacc8da15d1353392b026dd"
        );
    }

    #[test]
    fn digest_changes_for_each_field_and_handles_korean() {
        let base = oracle_digest("command", "cargo test -q", "ROUND_OK", ".");
        for changed in [
            oracle_digest("command", "cargo test -x", "ROUND_OK", "."),
            oracle_digest("command", "cargo test -q", "라운드_OK", "."),
            oracle_digest("command", "cargo test -q", "ROUND_OK", "crates"),
        ] {
            assert_ne!(changed, base);
        }
    }

    #[test]
    fn duplicate_fields_schema_and_blank_or_partial_lines_are_rejected() {
        for (tag, body) in [
            (
                "duplicate-field",
                "{\"kind\":\"schema\",\"version\":1,\"version\":1,\"round\":\"fixture-round\"}\n",
            ),
            (
                "duplicate-schema",
                "{\"kind\":\"schema\",\"version\":1,\"round\":\"fixture-round\"}\n{\"kind\":\"schema\",\"version\":1,\"round\":\"fixture-round\"}\n",
            ),
            (
                "blank-line",
                "{\"kind\":\"schema\",\"version\":1,\"round\":\"fixture-round\"}\n\n",
            ),
            (
                "partial",
                "{\"kind\":\"schema\",\"version\":1,\"round\":\"fixture-round\"}",
            ),
        ] {
            assert!(
                matches!(write_read(tag, body), Err(LedgerError::Schema(_))),
                "{tag}"
            );
        }
    }

    #[test]
    fn numeric_type_boundaries_are_rejected_outside_i32_and_u64() {
        let prefix = concat!(
            "{\"kind\":\"schema\",\"version\":1,\"round\":\"fixture-round\"}\n",
            "{\"kind\":\"oracle\",\"id\":\"A1\",\"mode\":\"command\",",
            "\"check\":\"x\",\"expect\":{\"literal\":\"y\"},\"cwd\":\".\"}\n",
        );
        let too_large_exit = format!(
            "{prefix}{{\"kind\":\"evidence\",\"id\":\"A1\",\"oracle_digest\":\"{}\",\"exit\":2147483648,\"matched\":true,\"output_digest\":\"{}\",\"output_bytes\":0}}\n",
            "a".repeat(64),
            "b".repeat(64),
        );
        let too_large_bytes = format!(
            "{prefix}{{\"kind\":\"evidence\",\"id\":\"A1\",\"oracle_digest\":\"{}\",\"exit\":0,\"matched\":true,\"output_digest\":\"{}\",\"output_bytes\":18446744073709551616}}\n",
            "a".repeat(64),
            "b".repeat(64),
        );
        for (tag, body) in [
            ("exit-over-i32", too_large_exit),
            ("bytes-over-u64", too_large_bytes),
        ] {
            assert!(
                matches!(write_read(tag, &body), Err(LedgerError::Schema(_))),
                "{tag}"
            );
        }
    }

    #[test]
    fn string_line_and_file_limits_fail_closed_at_the_upper_edge() {
        let exact = "x".repeat(ROUND_VERIFICATION_STRING_MAX_BYTES);
        assert_eq!(exact.len(), ROUND_VERIFICATION_STRING_MAX_BYTES);
        validate_string("literal", &exact, true).expect("32 KiB 이하");
        let over = format!("{exact}x");
        assert!(matches!(
            validate_string("literal", &over, true),
            Err(LedgerError::Schema(_))
        ));

        let long_line = format!("{}\n", " ".repeat(ROUND_VERIFICATION_LINE_MAX_BYTES + 1));
        assert!(matches!(
            write_read("long-line", &long_line),
            Err(LedgerError::Schema(_))
        ));

        let path = temp("large-file");
        std::fs::write(
            &path,
            vec![b' '; ROUND_VERIFICATION_FILE_MAX_BYTES as usize + 1],
        )
        .expect("large fixture");
        assert!(matches!(
            read(&path, "fixture-round"),
            Err(LedgerError::Schema(_))
        ));
    }

    #[test]
    fn crlf_has_the_same_schema_result() {
        let body = "{\"kind\":\"schema\",\"version\":1,\"round\":\"fixture-round\"}\r\n";
        let got = write_read("crlf", body).expect("CRLF ledger");
        assert!(got.conditions.is_empty());
    }

    #[test]
    fn schema_two_requires_projected_digest_but_schema_one_stays_readable() {
        let schema_one = concat!(
            "{\"kind\":\"schema\",\"version\":1,\"round\":\"fixture-round\"}\n",
            "{\"kind\":\"oracle\",\"id\":\"A1\",\"mode\":\"command\",",
            "\"check\":\"x\",\"expect\":{\"literal\":\"y\"},\"cwd\":\".\"}\n",
        );
        assert_eq!(
            write_read("schema-one-compatible", schema_one)
                .unwrap()
                .schema_version,
            1
        );

        let missing_projected = format!(
            "{{\"kind\":\"schema\",\"version\":2,\"round\":\"fixture-round\"}}\n\
             {{\"kind\":\"oracle\",\"id\":\"A1\",\"mode\":\"command\",\"check\":\"x\",\"expect\":{{\"literal\":\"y\"}},\"cwd\":\".\"}}\n\
             {{\"kind\":\"evidence\",\"id\":\"A1\",\"oracle_digest\":\"{}\",\"exit\":0,\"matched\":true,\"output_digest\":\"{}\",\"output_bytes\":1}}\n",
            "a".repeat(64),
            "b".repeat(64),
        );
        assert!(matches!(
            write_read("schema-two-projected", &missing_projected),
            Err(LedgerError::Schema(_))
        ));
    }
}
