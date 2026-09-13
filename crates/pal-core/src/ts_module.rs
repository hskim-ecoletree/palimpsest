//! TypeScript 모듈 지정자를 저장소의 파일로 편다 — **파일 간 해소의 TS 몫.**
//!
//! # 왜 따로 있나
//!
//! [`crate::cross_file_edges`] 의 모듈 경로 펴기는 Rust 의 규칙(`crate`·`self`·`super`·
//! 크레이트 이름)만 안다. TS 의 지정자는 `./fs` · `~/core/fs` 같은 **경로 문자열**이라
//! 그 규칙에 넣으면 첫 세그먼트가 크레이트가 아니어서 전부 「저장소 밖」으로 셌다
//! (2026-09-13 실측: 남의 TS 저장소에서 파일 간 해소 `0/11010`, 저장소 안 지정자 1,701).
//!
//! # 규칙은 TypeScript 의 것이다
//!
//! 이 모듈은 규칙을 **지어내지 않고 옮긴다.** 옳은지는 TypeScript 컴파일러
//! (`ts.resolveModuleName`)가 판정한다 — 회차 `2026-09-13-first-release-elsewhere` 의
//! `A3`·`A5` 가 그 대조다.
//!
//! | 지정자 | 무엇을 하나 |
//! |---|---|
//! | `node:…` | 저장소 밖 |
//! | `./…` · `../…` | 가져오는 파일의 자리에서 편다 |
//! | 그 밖 | 가장 가까운 `tsconfig.json` 의 `paths` → `baseUrl` 순. 둘 다 안 맞으면 **맨 지정자** |
//!
//! 파일로 펼 때의 순서(확장자 없음 · `.js` 대응 · 디렉터리 `index`)는 [`TsProject::load`] 가 진다.
//!
//! # 하지 않는 것
//!
//! - **`node_modules` 와 워크스페이스 패키지를 안 푼다.** 맨 지정자는 「저장소 밖」이 아니라
//!   **「패키지 이름 — 가리지 않았다」** 로 돌려준다. 워크스페이스라면 저장소 안이기 때문이다.
//! - **패키지 `extends`(`@tsconfig/…`)를 안 따라간다.** 그 설정 아래 파일의 별칭은
//!   「tsconfig 를 다 못 읽었다」로 돌려준다 — 「저장소 밖」으로 적으면 거짓이다.
//! - **`moduleResolution: classic`** 은 맞추지 않는다 — 그 모드는 따로 돌려준다.
//! - 디렉터리 안 `package.json` 의 `types`·`main` 은 안 본다 — `index` 만 본다.

// ⚠ **확장자는 대소문자를 가린다** — TypeScript 가 `.TS` 를 `.ts` 로 안 읽는다. 그래서
//   `case_sensitive_file_extension_comparisons` 를 이 모듈에서 끈다.
#![allow(clippy::case_sensitive_file_extension_comparisons)]

use std::collections::{BTreeMap, BTreeSet};

/// `compilerOptions.moduleResolution` — **TypeScript 가 기본값을 고르는 규칙까지 옮긴다.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum TsModuleResolution {
    /// `node` · `node10` — 확장자 없는 지정자와 디렉터리 `index` 를 푼다.
    #[default]
    Node10,
    /// `bundler` — 풀리는 꼴은 `node10` 과 같다.
    Bundler,
    /// `node16` · `nodenext` — **ESM 파일에서는** 확장자 없는 상대 경로를 안 푼다.
    Node16,
    /// `classic` — 맞추지 않는다.
    Classic,
}

impl TsModuleResolution {
    /// 설정의 두 값에서 모드를 고른다. `moduleResolution` 이 없으면 `module` 에서 유도한다
    /// (TypeScript 5 의 기본값 규칙).
    fn of(module_resolution: Option<&str>, module: Option<&str>) -> Self {
        if let Some(m) = module_resolution {
            return match m.to_ascii_lowercase().as_str() {
                "bundler" => Self::Bundler,
                "node16" | "nodenext" => Self::Node16,
                "classic" => Self::Classic,
                _ => Self::Node10,
            };
        }
        match module.map(str::to_ascii_lowercase).as_deref() {
            None | Some("commonjs") => Self::Node10,
            Some("node16" | "node18" | "node20" | "nodenext") => Self::Node16,
            Some("preserve") => Self::Bundler,
            Some(_) => Self::Classic,
        }
    }
}

/// 한 `tsconfig.json` 을 `extends` 까지 합친 것.
///
/// ⚠ **이 모듈 안에서만 쓴다.** `base_url` 의 `None` 은 「`baseUrl` 을 안 적었다」 한 뜻뿐이고
/// 이 모듈 안에서만 읽힌다 — 공개하면 「안 만듦」과 갈리지 않는 선택 필드가 된다(ADR-0005).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct TsConfig {
    /// `baseUrl` — 저장소 상대 경로로 이미 폈다.
    base_url: Option<String>,
    /// `paths` — `(패턴, 대체들)`. 대체는 **기준 자리를 이미 붙인** 저장소 상대 경로다.
    ///
    /// 기준 자리는 `baseUrl` 이 있으면 그것이고, 없으면 **`paths` 를 정의한 설정 파일의
    /// 자리**다 — 상속된 `paths` 를 자식의 자리로 풀면 틀린다(TypeScript 규칙).
    paths: Vec<(String, Vec<String>)>,
    resolution: TsModuleResolution,
    /// `extends` 사슬을 끝까지 못 읽었다 — 패키지 · 없는 파일 · 파싱 실패 · 순환.
    incomplete: bool,
}

/// 지정자 하나를 편 결과.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TsResolution {
    /// 저장소 안의 파일.
    File(String),
    /// 저장소 안을 가리키는데(상대 경로 · 맞은 별칭) 그 파일이 없다.
    NotFound,
    /// 저장소 밖이 확실하다 — `node:` · 뿌리 위로 올라가는 상대 경로 · 절대 경로.
    OutsideRepo,
    /// 맨 지정자 — 설치된 패키지인지 워크스페이스인지 가리지 않았다.
    Bare,
    /// 이 파일의 tsconfig 를 다 못 읽어서 별칭인지 모른다.
    ConfigIncomplete,
    /// 이 파일의 해소 모드를 맞추지 않는다(`classic`).
    UnsupportedMode,
}

/// 저장소 하나의 TS 해소 재료 — 파일 목록 · tsconfig · `package.json` 의 `type`.
#[derive(Debug, Clone, Default)]
pub struct TsProject {
    files: BTreeSet<String>,
    /// 설정 파일이 놓인 디렉터리 → 합친 설정. 뿌리는 `""`.
    configs: BTreeMap<String, TsConfig>,
    /// `package.json` 이 놓인 디렉터리 → `"type": "module"` 인가.
    module_scopes: BTreeMap<String, bool>,
}


impl TsProject {
    /// 트리의 파일 목록과 **그 트리에서 읽는 함수**로 세운다.
    ///
    /// `read` 는 저장소 상대 경로를 받아 내용을 돌려준다. 답이 선 트리(커밋 · 워킹트리)에서
    /// 읽어야 한다 — 워킹트리의 tsconfig 로 커밋된 소스를 풀면 두 답이 갈린다.
    pub fn build<I, F>(files: I, read: F) -> Self
    where
        I: IntoIterator<Item = String>,
        F: Fn(&str) -> Option<String>,
    {
        let files: BTreeSet<String> = files.into_iter().collect();
        let mut configs = BTreeMap::new();
        let mut module_scopes = BTreeMap::new();
        for f in &files {
            let (dir, name) = split_dir(f);
            if name == "tsconfig.json" {
                configs.insert(dir.to_owned(), load_config(f, &read));
            } else if name == "package.json" {
                let is_module = read(f)
                    .and_then(|t| parse_jsonc(&t))
                    .and_then(|v| v.get("type").and_then(|x| x.as_str()).map(|s| s == "module"))
                    .unwrap_or(false);
                module_scopes.insert(dir.to_owned(), is_module);
            }
        }
        Self { files, configs, module_scopes }
    }

    /// 이 재료가 파일 목록을 갖고 있는가 — 비었으면 부르는 쪽이 자기 목록을 댄다.
    #[must_use]
    pub fn has_files(&self) -> bool {
        !self.files.is_empty()
    }

    /// `from` 파일의 `spec` 을 편다. `exists` 는 파일이 저장소에 있는지 답한다.
    #[must_use]
    pub fn resolve(&self, from: &str, spec: &str, exists: &dyn Fn(&str) -> bool) -> TsResolution {
        let has = |p: &str| self.files.contains(p) || exists(p);
        if spec.starts_with("node:") || spec.starts_with('/') {
            return TsResolution::OutsideRepo;
        }
        let config = self.nearest_config(from);
        let mode = config.map_or(TsModuleResolution::default(), |c| c.resolution);
        if mode == TsModuleResolution::Classic {
            return TsResolution::UnsupportedMode;
        }
        let extensionless = !(mode == TsModuleResolution::Node16 && self.is_esm(from));

        if spec == "." || spec == ".." || spec.starts_with("./") || spec.starts_with("../") {
            let Some(base) = join(split_dir(from).0, spec) else {
                return TsResolution::OutsideRepo;
            };
            return Self::load(&base, extensionless, &has).map_or(TsResolution::NotFound, TsResolution::File);
        }

        let Some(config) = config else { return TsResolution::Bare };
        let mut 별칭이_맞았다 = false;
        if let Some((prefix_len, subs, star)) = match_paths(&config.paths, spec) {
            let _ = prefix_len;
            별칭이_맞았다 = true;
            for sub in subs {
                let target = sub.replacen('*', star, 1);
                if let Some(base) = join("", &target)
                    && let Some(hit) = Self::load(&base, extensionless, &has)
                {
                    return TsResolution::File(hit);
                }
            }
        }
        if let Some(base_url) = &config.base_url
            && let Some(base) = join(base_url, spec)
            && let Some(hit) = Self::load(&base, extensionless, &has)
        {
            return TsResolution::File(hit);
        }
        if 별칭이_맞았다 {
            TsResolution::NotFound
        } else if config.incomplete {
            TsResolution::ConfigIncomplete
        } else {
            TsResolution::Bare
        }
    }

    /// 편 경로 하나를 **파일로** — TypeScript 의 확장자 순서.
    ///
    /// | 지정자 끝 | 찾는 순서 |
    /// |---|---|
    /// | `.js` · `.ts` | `.ts` → `.tsx` → `.d.ts` |
    /// | `.jsx` · `.tsx` | `.tsx` → `.ts` → `.d.ts` |
    /// | `.mjs` · `.mts` | `.mts` → `.d.mts` |
    /// | `.cjs` · `.cts` | `.cts` → `.d.cts` |
    /// | 그 밖(확장자 없음) | `.ts` → `.tsx` → `.d.ts` → 디렉터리의 `index.ts` → `index.tsx` → `index.d.ts` — **`extensionless` 일 때만** |
    fn load(base: &str, extensionless: bool, has: &dyn Fn(&str) -> bool) -> Option<String> {
        let first = |cands: &[String]| cands.iter().find(|c| has(c)).cloned();
        let name = split_dir(base).1;
        let with = |stem: &str, exts: &[&str]| exts.iter().map(|e| format!("{stem}{e}")).collect::<Vec<_>>();
        for (ext, order) in [
            (".js", &[".ts", ".tsx", ".d.ts"][..]),
            (".ts", &[".ts", ".tsx", ".d.ts"][..]),
            (".jsx", &[".tsx", ".ts", ".d.ts"][..]),
            (".tsx", &[".tsx", ".ts", ".d.ts"][..]),
            (".mjs", &[".mts", ".d.mts"][..]),
            (".mts", &[".mts", ".d.mts"][..]),
            (".cjs", &[".cts", ".d.cts"][..]),
            (".cts", &[".cts", ".d.cts"][..]),
        ] {
            if name.len() > ext.len() && name.ends_with(ext) {
                let stem = &base[..base.len() - ext.len()];
                return first(&with(stem, order));
            }
        }
        if !extensionless {
            return None;
        }
        first(&with(base, &[".ts", ".tsx", ".d.ts"]))
            .or_else(|| first(&with(&format!("{base}/index"), &[".ts", ".tsx", ".d.ts"])))
    }

    fn nearest_config(&self, from: &str) -> Option<&TsConfig> {
        ancestors(split_dir(from).0).find_map(|d| self.configs.get(d))
    }

    /// `node16` 모드에서 이 파일이 ESM 인가 — `.mts` · `.cts` 는 확장자가 정하고, 나머지는
    /// 가장 가까운 `package.json` 의 `type` 이 정한다.
    fn is_esm(&self, from: &str) -> bool {
        if from.ends_with(".mts") {
            return true;
        }
        if from.ends_with(".cts") {
            return false;
        }
        ancestors(split_dir(from).0).find_map(|d| self.module_scopes.get(d)).copied().unwrap_or(false)
    }
}

/// 설정 파일 하나를 `extends` 까지 합친다.
fn load_config(path: &str, read: &dyn Fn(&str) -> Option<String>) -> TsConfig {
    let mut raw = Raw::default();
    let mut seen = BTreeSet::new();
    collect(path, read, 0, &mut seen, &mut raw);
    let base_url = raw.base_url.as_ref().and_then(|(v, dir)| join(dir, v));
    let paths = raw
        .paths
        .as_ref()
        .map(|(map, dir)| {
            let root = base_url.clone().or_else(|| Some(dir.clone())).unwrap_or_default();
            map.iter()
                .map(|(pat, subs)| {
                    let subs = subs.iter().filter_map(|s| join(&root, s)).collect();
                    (pat.clone(), subs)
                })
                .collect()
        })
        .unwrap_or_default();
    TsConfig {
        base_url,
        paths,
        resolution: TsModuleResolution::of(raw.module_resolution.as_deref(), raw.module.as_deref()),
        incomplete: raw.incomplete,
    }
}

/// `paths` 의 항목들과 그것을 정의한 설정 파일의 자리.
type PathsFrom = (Vec<(String, Vec<String>)>, String);

/// 합치는 중간값 — 값마다 **정의한 설정 파일의 자리**를 함께 든다.
#[derive(Default)]
struct Raw {
    base_url: Option<(String, String)>,
    paths: Option<PathsFrom>,
    module_resolution: Option<String>,
    module: Option<String>,
    incomplete: bool,
}

fn collect(
    path: &str,
    read: &dyn Fn(&str) -> Option<String>,
    depth: usize,
    seen: &mut BTreeSet<String>,
    raw: &mut Raw,
) {
    if depth > crate::PROVISIONAL_TS_EXTENDS_DEPTH_MAX || !seen.insert(path.to_owned()) {
        raw.incomplete = true;
        return;
    }
    let Some(value) = read(path).and_then(|t| parse_jsonc(&t)) else {
        raw.incomplete = true;
        return;
    };
    let dir = split_dir(path).0.to_owned();

    // ① 부모를 먼저 — 자식이 덮어쓴다. 배열 `extends` 는 뒤의 것이 이긴다.
    let parents: Vec<String> = match value.get("extends") {
        Some(serde_json::Value::String(s)) => vec![s.clone()],
        Some(serde_json::Value::Array(a)) => a.iter().filter_map(|x| x.as_str().map(str::to_owned)).collect(),
        _ => Vec::new(),
    };
    for p in parents {
        if !(p.starts_with("./") || p.starts_with("../")) {
            // 패키지 `extends` — 따라가지 않는다.
            raw.incomplete = true;
            continue;
        }
        let file = if p.ends_with(".json") { p } else { format!("{p}.json") };
        match join(&dir, &file) {
            Some(target) => collect(&target, read, depth + 1, seen, raw),
            None => raw.incomplete = true,
        }
    }

    // ② 자기 값.
    let Some(opts) = value.get("compilerOptions").and_then(|x| x.as_object()) else { return };
    if let Some(b) = opts.get("baseUrl").and_then(|x| x.as_str()) {
        raw.base_url = Some((b.to_owned(), dir.clone()));
    }
    if let Some(map) = opts.get("paths").and_then(|x| x.as_object()) {
        let entries = map
            .iter()
            .map(|(k, v)| {
                let subs = v
                    .as_array()
                    .map(|a| a.iter().filter_map(|x| x.as_str().map(str::to_owned)).collect())
                    .unwrap_or_default();
                (k.clone(), subs)
            })
            .collect();
        raw.paths = Some((entries, dir.clone()));
    }
    if let Some(m) = opts.get("moduleResolution").and_then(|x| x.as_str()) {
        raw.module_resolution = Some(m.to_owned());
    }
    if let Some(m) = opts.get("module").and_then(|x| x.as_str()) {
        raw.module = Some(m.to_owned());
    }
}

/// `paths` 에서 맞은 것 — `(접두 길이, 대체들, 별이 맞은 조각)`.
type PathsHit<'a> = (usize, &'a [String], &'a str);

/// `paths` 에서 맞는 패턴 — **접두가 가장 긴 것**이 이긴다. 정확한 패턴이 먼저다.
fn match_paths<'a>(paths: &'a [(String, Vec<String>)], spec: &'a str) -> Option<PathsHit<'a>> {
    if let Some((_, subs)) = paths.iter().find(|(p, _)| !p.contains('*') && p == spec) {
        return Some((spec.len(), subs.as_slice(), ""));
    }
    let mut best: Option<PathsHit<'a>> = None;
    for (pat, subs) in paths {
        let Some((prefix, suffix)) = pat.split_once('*') else { continue };
        if spec.len() >= prefix.len() + suffix.len() && spec.starts_with(prefix) && spec.ends_with(suffix) {
            let star = &spec[prefix.len()..spec.len() - suffix.len()];
            if best.is_none_or(|(len, _, _)| prefix.len() > len) {
                best = Some((prefix.len(), subs.as_slice(), star));
            }
        }
    }
    best
}

/// 주석(`//` · `/* */`)과 꼬리 쉼표를 벗기고 JSON 으로 읽는다 — **tsconfig 는 JSONC 다.**
fn parse_jsonc(text: &str) -> Option<serde_json::Value> {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    let mut in_str = false;
    while i < chars.len() {
        let c = chars[i];
        if in_str {
            out.push(c);
            if c == '\\' && i + 1 < chars.len() {
                out.push(chars[i + 1]);
                i += 2;
                continue;
            }
            if c == '"' {
                in_str = false;
            }
            i += 1;
            continue;
        }
        match (c, chars.get(i + 1)) {
            ('"', _) => {
                in_str = true;
                out.push(c);
                i += 1;
            }
            ('/', Some('/')) => {
                while i < chars.len() && chars[i] != '\n' {
                    i += 1;
                }
            }
            ('/', Some('*')) => {
                i += 2;
                while i + 1 < chars.len() && !(chars[i] == '*' && chars[i + 1] == '/') {
                    i += 1;
                }
                i += 2;
            }
            _ => {
                out.push(c);
                i += 1;
            }
        }
    }
    // 꼬리 쉼표 — 문자열 밖에서 `,` 뒤에 공백만 있고 `}`·`]` 가 오면 지운다.
    let chars: Vec<char> = out.chars().collect();
    let mut clean = String::with_capacity(out.len());
    let mut in_str = false;
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if in_str {
            clean.push(c);
            if c == '\\' && i + 1 < chars.len() {
                clean.push(chars[i + 1]);
                i += 2;
                continue;
            }
            if c == '"' {
                in_str = false;
            }
            i += 1;
            continue;
        }
        if c == '"' {
            in_str = true;
        } else if c == ',' {
            let mut j = i + 1;
            while j < chars.len() && chars[j].is_whitespace() {
                j += 1;
            }
            if j < chars.len() && (chars[j] == '}' || chars[j] == ']') {
                i += 1;
                continue;
            }
        }
        clean.push(c);
        i += 1;
    }
    serde_json::from_str(&clean).ok()
}

/// `a/b/c.ts` → `("a/b", "c.ts")`. 뿌리의 파일은 `("", 이름)`.
fn split_dir(path: &str) -> (&str, &str) {
    path.rsplit_once('/').unwrap_or(("", path))
}

/// `dir` 에서 뿌리까지 — `a/b` → `a/b` · `a` · `""`.
fn ancestors(dir: &str) -> impl Iterator<Item = &str> {
    let mut next = Some(dir);
    std::iter::from_fn(move || {
        let cur = next?;
        next = if cur.is_empty() { None } else { Some(cur.rsplit_once('/').map_or("", |(p, _)| p)) };
        Some(cur)
    })
}

/// `dir` 에 `rel` 을 붙이고 `.`·`..` 를 편다. **뿌리 위로 올라가면 `None`** — 저장소 밖이다.
fn join(dir: &str, rel: &str) -> Option<String> {
    let mut segs: Vec<&str> = dir.split('/').filter(|s| !s.is_empty()).collect();
    for s in rel.split('/') {
        match s {
            "" | "." => {}
            ".." => {
                segs.pop()?;
            }
            _ => segs.push(s),
        }
    }
    Some(segs.join("/"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn 프로젝트(files: &[(&str, &str)]) -> TsProject {
        let map: BTreeMap<String, String> =
            files.iter().map(|(p, t)| ((*p).to_owned(), (*t).to_owned())).collect();
        TsProject::build(map.keys().cloned(), |p| map.get(p).cloned())
    }

    fn 편다(p: &TsProject, from: &str, spec: &str) -> TsResolution {
        p.resolve(from, spec, &|_| false)
    }

    fn 파일(s: &str) -> TsResolution {
        TsResolution::File(s.to_owned())
    }

    #[test]
    fn 상대_경로는_확장자_없이도_ts_로_펴지고_디렉터리는_index_로_간다() {
        let p = 프로젝트(&[("src/a.ts", ""), ("src/b/c.ts", ""), ("src/dir/index.ts", ""), ("src/x.ts", "")]);
        assert_eq!(편다(&p, "src/main.ts", "./a"), 파일("src/a.ts"));
        assert_eq!(편다(&p, "src/b/main.ts", "../a"), 파일("src/a.ts"));
        assert_eq!(편다(&p, "src/main.ts", "./b/c"), 파일("src/b/c.ts"));
        assert_eq!(편다(&p, "src/main.ts", "./dir"), 파일("src/dir/index.ts"));
        assert_eq!(편다(&p, "src/main.ts", "./x.js"), 파일("src/x.ts"));
        assert_eq!(편다(&p, "src/main.ts", "./x.ts"), 파일("src/x.ts"));
    }

    #[test]
    fn 확장자_순서가_typescript_와_같다() {
        let p = 프로젝트(&[("x.ts", ""), ("x.d.ts", ""), ("x.tsx", ""), ("x/index.ts", ""), ("y.tsx", ""), ("y.ts", "")]);
        assert_eq!(편다(&p, "m.ts", "./x"), 파일("x.ts"), "파일이 디렉터리 index 보다 먼저다");
        assert_eq!(편다(&p, "m.ts", "./y.jsx"), 파일("y.tsx"), ".jsx 는 .tsx 가 먼저다");
        assert_eq!(편다(&p, "m.ts", "./y.js"), 파일("y.ts"), ".js 는 .ts 가 먼저다");
    }

    #[test]
    fn 저장소_밖과_맨_지정자와_없는_파일을_가른다() {
        let p = 프로젝트(&[("src/a.ts", "")]);
        assert_eq!(편다(&p, "src/a.ts", "node:fs"), TsResolution::OutsideRepo);
        assert_eq!(편다(&p, "src/a.ts", "../../up"), TsResolution::OutsideRepo);
        assert_eq!(편다(&p, "src/a.ts", "zod"), TsResolution::Bare);
        assert_eq!(편다(&p, "src/a.ts", "./missing"), TsResolution::NotFound);
    }

    #[test]
    fn paths_별칭은_jsonc_와_baseurl_을_읽고_가장_긴_접두가_이긴다() {
        let p = 프로젝트(&[
            ("tsconfig.json", "{ // 주석\n \"compilerOptions\": { \"baseUrl\": \".\", /* 블록 */ \"paths\": { \"~/*\": [\"./src/*\"], \"~/core/*\": [\"./core/*\"], }, }, }"),
            ("src/fs.ts", ""),
            ("core/fs.ts", ""),
        ]);
        assert_eq!(편다(&p, "src/a.ts", "~/fs"), 파일("src/fs.ts"));
        assert_eq!(편다(&p, "src/a.ts", "~/core/fs"), 파일("core/fs.ts"));
        assert_eq!(편다(&p, "src/a.ts", "~/nothing"), TsResolution::NotFound, "맞은 별칭의 파일이 없다");
    }

    #[test]
    fn baseurl_없는_paths_는_정의한_설정의_자리가_기준이고_상속돼도_그렇다() {
        let p = 프로젝트(&[
            ("tsconfig.base.json", "{\"compilerOptions\":{\"paths\":{\"@lib/*\":[\"lib/*\"]}}}"),
            ("app/tsconfig.json", "{\"extends\":\"../tsconfig.base.json\"}"),
            ("lib/util.ts", ""),
        ]);
        assert_eq!(편다(&p, "app/main.ts", "@lib/util"), 파일("lib/util.ts"));
    }

    #[test]
    fn 가장_가까운_tsconfig_가_이긴다() {
        let p = 프로젝트(&[
            ("tsconfig.json", "{\"compilerOptions\":{\"paths\":{\"@/*\":[\"root/*\"]}}}"),
            ("pkg/tsconfig.json", "{\"compilerOptions\":{\"paths\":{\"@/*\":[\"inner/*\"]}}}"),
            ("root/a.ts", ""),
            ("pkg/inner/a.ts", ""),
        ]);
        assert_eq!(편다(&p, "pkg/src/m.ts", "@/a"), 파일("pkg/inner/a.ts"));
        assert_eq!(편다(&p, "other/m.ts", "@/a"), 파일("root/a.ts"));
    }

    #[test]
    fn 패키지_extends_아래의_맨_지정자는_저장소_밖이_아니라_못_읽음이다() {
        let p = 프로젝트(&[("tsconfig.json", "{\"extends\":\"@tsconfig/node20/tsconfig.json\"}"), ("src/a.ts", "")]);
        assert_eq!(편다(&p, "src/a.ts", "@/a"), TsResolution::ConfigIncomplete);
        assert_eq!(편다(&p, "src/a.ts", "./a"), 파일("src/a.ts"), "상대 경로는 설정과 무관하다");
    }

    #[test]
    fn nodenext_의_esm_파일은_확장자_없는_상대_경로를_안_편다() {
        let p = 프로젝트(&[
            ("tsconfig.json", "{\"compilerOptions\":{\"module\":\"nodenext\"}}"),
            ("package.json", "{\"type\":\"module\"}"),
            ("src/x.ts", ""),
        ]);
        assert_eq!(편다(&p, "src/m.ts", "./x"), TsResolution::NotFound);
        assert_eq!(편다(&p, "src/m.ts", "./x.js"), 파일("src/x.ts"));
        assert_eq!(편다(&p, "src/m.cts", "./x"), 파일("src/x.ts"), ".cts 는 CJS 다");
    }

    #[test]
    fn classic_은_맞추지_않는다고_말한다() {
        let p = 프로젝트(&[("tsconfig.json", "{\"compilerOptions\":{\"module\":\"esnext\"}}"), ("a.ts", "")]);
        assert_eq!(편다(&p, "m.ts", "./a"), TsResolution::UnsupportedMode);
    }

    #[test]
    fn 뿌리의_파일도_경로_계산이_맞다() {
        assert_eq!(join("", "./a"), Some("a".to_owned()));
        assert_eq!(join("a/b", "../../c"), Some("c".to_owned()));
        assert_eq!(join("a", "../../c"), None);
        assert_eq!(ancestors("a/b").collect::<Vec<_>>(), vec!["a/b", "a", ""]);
    }
}
