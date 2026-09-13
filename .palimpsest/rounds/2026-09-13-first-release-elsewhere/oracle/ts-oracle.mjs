#!/usr/bin/env node
// TypeScript 컴파일러 대조 — 회차 `2026-09-13-first-release-elsewhere` 의 `A3` · `A3-a` · `A5`.
//
// ★ 이 스크립트는 **해소 규칙을 스스로 갖지 않는다.** 지정자를 파일로 펴는 것
//   (`ts.resolveModuleName`), 해소 모드(`ts.getModeForUsageLocation`), 이름의 참조
//   (`TypeChecker.getSymbolAtLocation`)는 전부 TypeScript 가 답한다. `pal` 에게서 받는 것은
//   파일 간 엣지(`pal query graph.dump --json`)와 못 푼 참조(`pal export --format cypher`)뿐이다.
//
// 사용:
//   node ts-oracle.mjs --repo=<저장소> --pal=<pal 바이너리> [--typescript=<typescript 모듈 경로>]
//                      [--unresolved] [--recall] [--corrupt]
//   --unresolved  A5 ⑵ — `outside_repo` 로 남은 못 푼 참조 중 TypeScript 가 저장소 안으로 푸는 것
//   --recall      A5 ⑷ — TypeScript 가 푸는 저장소 안 이름 임포트 중 `pal` 의 엣지가 빠진 것
//   --corrupt     A3-a — 엣지 하나의 대상 파일을 바꿔 넣는다(음성 대조: 불일치 ≥ 1 이어야 한다)
//
// 종료 코드: 불일치·빠짐·위반이 0 이면 0, 아니면 1. **판정은 산출이 한다 — 종료 코드는 편의다.**

import { createRequire } from 'node:module';
import { execFileSync } from 'node:child_process';
import path from 'node:path';
import fs from 'node:fs';

const args = Object.fromEntries(
  process.argv.slice(2).map((a) => {
    const [k, ...v] = a.replace(/^--/, '').split('=');
    return [k, v.length ? v.join('=') : true];
  }),
);
const root = fs.realpathSync(path.resolve(args.repo));
const pal = args.pal;
const tsPath = args.typescript || path.join(process.env.HOME, 'dev/projects/ditto/node_modules/typescript');
const require = createRequire(import.meta.url);
const ts = require(tsPath);

const sh = (cmd, argv) => execFileSync(cmd, argv, { cwd: root, encoding: 'utf8', maxBuffer: 1 << 30 });
const rel = (p) => path.relative(root, p).split(path.sep).join('/');
const isTs = (p) => /\.(ts|tsx|mts|cts)$/.test(p);
const out = [];
const say = (s = '') => out.push(s);

say(`# TypeScript 컴파일러 대조 — ${path.basename(root)}`);
say('');
say(`- pal: \`${sh(pal, ['--version']).trim()}\``);
say(`- typescript: \`${ts.version}\` (${tsPath.replace(process.env.HOME, '~')})`);
say(`- 저장소 HEAD: \`${sh('git', ['rev-parse', '--short', 'HEAD']).trim()}\``);
say(`- 모드: ${['unresolved', 'recall', 'corrupt'].filter((k) => args[k]).join(' · ') || '정밀도만'}`);
say('');

// ── ① pal 의 파일 간 엣지 ────────────────────────────────────────────────────
const dump = JSON.parse(sh(pal, ['query', 'graph.dump', '--json'])).answer;
const nodes = new Map(dump.nodes.map((n) => [n.id, n]));
let edges = dump.edges
  .map((e) => [nodes.get(e.from), nodes.get(e.to)])
  .filter(([a, b]) => a && b && a.path !== b.path && isTs(a.path));
if (args.corrupt && edges.length) {
  const [from, to] = edges[0];
  const other = dump.nodes.find((n) => n.path !== to.path && n.path !== from.path);
  if (other) edges[0] = [from, { ...to, path: other.path }];
}

// ── ② TypeScript 설정 — 파일마다 가장 가까운 tsconfig(저장소 안에서만) ────────
const configs = new Map();
function configFor(file) {
  const dir = path.dirname(path.join(root, file));
  let cfgPath = ts.findConfigFile(dir, ts.sys.fileExists, 'tsconfig.json');
  if (cfgPath && !fs.realpathSync(cfgPath).startsWith(root + path.sep)) cfgPath = undefined;
  const key = cfgPath || '(없음)';
  if (!configs.has(key)) {
    let options = {};
    let inherited = false;
    if (cfgPath) {
      const host = { ...ts.sys, onUnRecoverableConfigFileDiagnostic: () => {} };
      const parsed = ts.getParsedCommandLineOfConfigFile(cfgPath, {}, host);
      options = parsed ? parsed.options : {};
      const raw = ts.readConfigFile(cfgPath, ts.sys.readFile).config || {};
      inherited = Boolean(raw.extends) && !(raw.compilerOptions && raw.compilerOptions.paths);
    }
    configs.set(key, { options, cfgPath, inherited });
  }
  return configs.get(key);
}
const moduleCache = ts.createModuleResolutionCache(root, (x) => x);
const modeLabel = (o) => {
  const k = o.moduleResolution;
  if (k === ts.ModuleResolutionKind.Bundler) return 'bundler';
  if (k === ts.ModuleResolutionKind.Node16 || k === ts.ModuleResolutionKind.NodeNext) return 'nodenext';
  if (k === ts.ModuleResolutionKind.Node10) return 'node10';
  return `기본(${k ?? '없음'})`;
};

// ── ③ 소스 파일과 임포트 ─────────────────────────────────────────────────────
const sources = new Map();
function sourceOf(file) {
  if (!sources.has(file)) {
    const abs = path.join(root, file);
    const { options } = configFor(file);
    const format = ts.getImpliedNodeFormatForFile(abs, moduleCache.getPackageJsonInfoCache(), ts.sys, options);
    const text = fs.readFileSync(abs, 'utf8');
    sources.set(file, ts.createSourceFile(abs, text, { languageVersion: ts.ScriptTarget.Latest, impliedNodeFormat: format }, true));
  }
  return sources.get(file);
}
function importsOf(file) {
  const sf = sourceOf(file);
  const outList = [];
  for (const st of sf.statements) {
    if (!ts.isImportDeclaration(st) || !ts.isStringLiteral(st.moduleSpecifier)) continue;
    const nb = st.importClause && st.importClause.namedBindings;
    if (!nb || !ts.isNamedImports(nb)) continue;
    outList.push({
      decl: st,
      lit: st.moduleSpecifier,
      spec: st.moduleSpecifier.text,
      names: nb.elements.map((el) => ({ imported: (el.propertyName || el.name).text, el })),
    });
  }
  return outList;
}
function resolve(file, imp) {
  const { options } = configFor(file);
  const sf = sourceOf(file);
  const mode = ts.getModeForUsageLocation(sf, imp.lit, options);
  const r = ts.resolveModuleName(imp.spec, path.join(root, file), options, ts.sys, moduleCache, undefined, mode).resolvedModule;
  if (!r || r.isExternalLibraryImport) return null;
  const abs = fs.realpathSync(r.resolvedFileName);
  return abs.startsWith(root + path.sep) ? rel(abs) : null;
}

// ── ④ 가지 — 표본 수를 센다. 0 인 가지는 이름으로 적는다 ─────────────────────
const 지정자가지 = ['확장자 없음', '.js 대응', '명시 .ts', '디렉터리 index', '별칭', '상속 별칭'];
const 대상가지 = ['.ts', '.tsx', '.d.ts'];
const 모드가지 = ['bundler', 'nodenext'];
const 표본 = {};
const bump = (k) => (표본[k] = (표본[k] || 0) + 1);
function branches(file, spec, target) {
  const cfg = configFor(file);
  let b;
  if (!spec.startsWith('.')) b = cfg.inherited ? '상속 별칭' : '별칭';
  else if (/\.(js|jsx|mjs|cjs)$/.test(spec)) b = '.js 대응';
  else if (/\.(ts|tsx|mts|cts)$/.test(spec)) b = '명시 .ts';
  else if (/\/index\.(d\.ts|tsx|ts)$/.test(target) && !/\/index$/.test(spec)) b = '디렉터리 index';
  else b = '확장자 없음';
  const t = target.endsWith('.d.ts') ? '.d.ts' : target.endsWith('.tsx') ? '.tsx' : '.ts';
  return [b, t, modeLabel(cfg.options)];
}

// ── ⑤ 정밀도 — 선 엣지의 대상 파일이 TypeScript 의 답과 같은가 ───────────────
const 불일치 = [];
let 잰_엣지 = 0;
for (const [from, to] of edges) {
  const imps = importsOf(from.path).filter((i) => i.names.some((n) => n.imported === to.name));
  잰_엣지 += 1;
  if (!imps.length) {
    불일치.push(`${from.path} → ${to.path}#${to.name} — 그 이름을 들이는 이름 임포트가 없다`);
    continue;
  }
  const hits = imps.map((i) => resolve(from.path, i));
  if (!hits.includes(to.path)) {
    불일치.push(`${from.path} → ${to.path}#${to.name} — TypeScript: ${hits.map((h) => h ?? '(저장소 안으로 못 풂)').join(', ')}`);
  } else {
    imps.forEach((i, k) => {
      if (hits[k] === to.path) branches(from.path, i.spec, to.path).forEach(bump);
    });
  }
}
say('## 정밀도 — 파일 간 엣지의 대상 파일');
say('');
say(`- 잰 엣지 ${잰_엣지} · 불일치 **${불일치.length}**`);
for (const m of 불일치.slice(0, 50)) say(`  - ${m}`);
if (불일치.length > 50) say(`  - … 그 밖 ${불일치.length - 50}`);
say('');

// ── ⑥ A5 ⑵ — `outside_repo` 인데 TypeScript 가 저장소 안으로 푸는 것 ───────────
const 위반 = [];
const 꼴위반 = [];
// 조건 문면의 꼴 — 지정자가 `./` · `../` 이거나 그 파일에 걸린 tsconfig `paths` 의 열쇠에 맞는다.
// ⚠ TypeScript 가 푸는가와 **다른 술어다**: 꼴은 상대·별칭인데 TypeScript 도 못 푸는 지정자(없는 파일)가
//   `outside_repo` 로 남으면 여기서만 걸린다.
function 별칭에_맞나(file, spec) {
  const paths = configFor(file).options.paths || {};
  return Object.keys(paths).some((k) => (k.endsWith('*') ? spec.startsWith(k.slice(0, -1)) : spec === k));
}
if (args.unresolved) {
  const cy = sh(pal, ['export', '--format', 'cypher']);
  const re = /CREATE \(:UnresolvedRef \{site: "([^"]+)", name: "((?:[^"\\]|\\.)*)", reason: "([a-z_]+)"/g;
  const byName = new Map(dump.nodes.map((n) => [n.id, n]));
  let 셈 = 0;
  for (const m of cy.matchAll(re)) {
    if (m[3] !== 'outside_repo') continue;
    const site = byName.get(m[1]);
    if (!site || !isTs(site.path)) continue;
    셈 += 1;
    const name = JSON.parse(`"${m[2]}"`);
    for (const imp of importsOf(site.path).filter((i) => i.names.some((n) => n.imported === name))) {
      const target = resolve(site.path, imp);
      if (target) 위반.push(`${site.path} · \`${imp.spec}\` · ${name} → TypeScript: ${target}`);
      if (imp.spec.startsWith('./') || imp.spec.startsWith('../') || 별칭에_맞나(site.path, imp.spec)) {
        꼴위반.push(`${site.path} · \`${imp.spec}\` · ${name}`);
      }
    }
  }
  say('## A5 ⑵ — `outside_repo` 로 남은 못 푼 참조');
  say('');
  say(`- TS 파일의 \`outside_repo\` ${셈} · 그중 TypeScript 가 저장소 안 파일로 푸는 것 **${위반.length}**`);
  for (const v of 위반.slice(0, 50)) say(`  - ${v}`);
  say(`- 그중 지정자가 \`./\` · \`../\` · tsconfig 별칭 꼴인 것(조건 문면) **${꼴위반.length}**`);
  for (const v of 꼴위반.slice(0, 50)) say(`  - ${v}`);
  say('');
}

// ── ⑦ A5 ⑷ — 재현율 ─────────────────────────────────────────────────────────
const 빠짐 = [];
if (args.recall) {
  const files = sh('git', ['ls-files', '*.ts', '*.tsx', '*.mts', '*.cts']).split('\n').filter(Boolean);
  const 있음 = new Set(edges.map(([a, b]) => `${a.path}|${b.path}|${b.name}`));
  const 뺀몫 = { '대상이 그 이름을 선언하지 않는다(재수출로만 진다)': 0, '참조가 파일 최상위에만 있다': 0, '참조가 없다': 0 };
  let 모집단 = 0;
  const groups = new Map();
  for (const f of files) {
    const key = configFor(f).cfgPath || '(없음)';
    if (!groups.has(key)) groups.set(key, []);
    groups.get(key).push(f);
  }
  const declares = (file, name) =>
    sourceOf(file).statements.some((st) => {
      if (ts.isVariableStatement(st)) return st.declarationList.declarations.some((d) => ts.isIdentifier(d.name) && d.name.text === name);
      return st.name && ts.isIdentifier(st.name) && st.name.text === name;
    });
  const isDeclarationStatement = (st) =>
    ts.isFunctionDeclaration(st) || ts.isClassDeclaration(st) || ts.isInterfaceDeclaration(st) ||
    ts.isTypeAliasDeclaration(st) || ts.isEnumDeclaration(st) || ts.isVariableStatement(st) || ts.isModuleDeclaration(st);
  for (const [, groupFiles] of groups) {
    const { options } = configFor(groupFiles[0]);
    const program = ts.createProgram(groupFiles.map((f) => path.join(root, f)), { ...options, noEmit: true });
    const checker = program.getTypeChecker();
    for (const f of groupFiles) {
      const sf = program.getSourceFile(path.join(root, f));
      if (!sf) continue;
      const local = importsOf(f);
      for (const imp of local) {
        const target = resolve(f, imp);
        if (!target || !isTs(target)) continue;
        for (const n of imp.names) {
          // `importsOf` 는 따로 만든 소스 트리라 체커가 모른다 — 프로그램의 트리에서 같은 자리를 찾는다.
          const el = findAt(sf, n.el.name.getStart());
          const alias = el && checker.getSymbolAtLocation(el);
          if (!alias) continue;
          if (!declares(target, n.imported)) {
            뺀몫['대상이 그 이름을 선언하지 않는다(재수출로만 진다)'] += 1;
            continue;
          }
          let 선언안 = 0;
          let 최상위 = 0;
          const visit = (node, top) => {
            if (ts.isImportDeclaration(node)) return;
            if (ts.isIdentifier(node) && checker.getSymbolAtLocation(node) === alias) {
              if (top && isDeclarationStatement(top)) 선언안 += 1;
              else 최상위 += 1;
            }
            ts.forEachChild(node, (c) => visit(c, top ?? (node === sf ? c : top)));
          };
          for (const st of sf.statements) visit(st, st);
          if (선언안 > 0) {
            모집단 += 1;
            if (!있음.has(`${f}|${target}|${n.imported}`)) {
              const { line } = sf.getLineAndCharacterOfPosition(imp.decl.getStart());
              빠짐.push(`${f}:${line + 1} · \`${imp.spec}\` · ${n.imported} → ${target}`);
            }
          } else if (최상위 > 0) {
            뺀몫['참조가 파일 최상위에만 있다'] += 1;
          } else {
            뺀몫['참조가 없다'] += 1;
          }
        }
      }
    }
  }
  say('## A5 ⑷ — 재현율');
  say('');
  say(`- 모집단(TypeScript 가 저장소 안으로 풀고 · 대상이 그 이름을 선언하고 · 선언 안에서 참조되는 이름 임포트) **${모집단}** · pal 엣지가 빠진 것 **${빠짐.length}**`);
  for (const [k, v] of Object.entries(뺀몫)) say(`- 모집단 밖 — ${k}: ${v}`);
  for (const m of 빠짐.slice(0, 80)) say(`  - ${m}`);
  if (빠짐.length > 80) say(`  - … 그 밖 ${빠짐.length - 80}`);
  say('');
}

function findAt(sf, pos) {
  let hit;
  const walk = (node) => {
    if (node.getStart(sf) === pos && ts.isIdentifier(node)) hit = node;
    if (!hit && node.pos <= pos && pos < node.end) ts.forEachChild(node, walk);
  };
  walk(sf);
  return hit;
}

// ── ⑧ 가지별 표본 ───────────────────────────────────────────────────────────
say('## 가지별 표본 — 정밀도에서 일치한 임포트');
say('');
for (const [이름, 가지들] of [['지정자', 지정자가지], ['대상 확장자', 대상가지], ['모드', 모드가지]]) {
  say(`- ${이름}: ${가지들.map((g) => `${g} ${표본[g] || 0}`).join(' · ')}`);
}
const 영 = [...지정자가지, ...대상가지, ...모드가지].filter((g) => !표본[g]);
say(`- **표본 0 인 가지**: ${영.length ? 영.join(' · ') : '없음'}`);
say('');

console.log(out.join('\n'));
process.exitCode = 불일치.length + 위반.length + 빠짐.length > 0 ? 1 : 0;
