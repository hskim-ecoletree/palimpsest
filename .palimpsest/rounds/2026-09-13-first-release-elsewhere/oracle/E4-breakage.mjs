#!/usr/bin/env node
// `E4` — 깨질 곳의 대조. 회차 `2026-09-13-first-release-elsewhere`.
//
// P = 변경 A(파일 삭제만) 뒤 `tsc --noEmit` 에 **새로 오류가 난 파일** 집합
// C = 봉인 심볼들의 `pal query symbol.callers` 가 호출자로 낸 파일 집합
// D = 과제가 지운 파일
//
// 합격선: P∖(C∪D) 의 파일마다, 그 파일의 **새 오류 자리가 전부 파일 최상위 문장**(import · 식 문장 ·
// export 선언)에 있다 — 화면이 「호출자 수는 하한 · 파일 최상위의 참조는 세지 않는다」고 말한 꼴 밖이 0.
// 오류 자리가 최상위 **선언**(함수 · 클래스 · 변수 · 인터페이스 · 타입 · enum · 모듈) 안이면 그것은
// `pal` 이 호출자로 댔어야 할 자리다.
//
// 자리를 가르는 것은 TypeScript 의 파서다. `tsc` 가 낸 (파일, 줄, 열)을 그대로 쓴다.
//
// 사용: node E4-breakage.mjs --repo=<복제본> --before=<tsc 전 출력> --after=<tsc A 뒤 출력>
//                            --callers=<04-callers 디렉터리> --deleted=<지운 파일> [--typescript=<경로>]
import { createRequire } from 'node:module';
import path from 'node:path';
import fs from 'node:fs';

const args = Object.fromEntries(process.argv.slice(2).map((a) => {
  const [k, ...v] = a.replace(/^--/, '').split('=');
  return [k, v.join('=')];
}));
const ts = createRequire(import.meta.url)(args.typescript || path.join(process.env.HOME, 'dev/projects/ditto/node_modules/typescript'));
const root = path.resolve(args.repo);

// tsc 출력: `src/a.ts(12,5): error TS2304: …`
const errors = (file) => fs.readFileSync(file, 'utf8').split('\n')
  .map((l) => l.match(/^(.+?)\((\d+),(\d+)\): error (TS\d+): (.*)$/))
  .filter(Boolean)
  .map((m) => ({ file: m[1], line: Number(m[2]), col: Number(m[3]), code: m[4], msg: m[5] }));
const before = errors(args.before);
const after = errors(args.after);
const beforeKeys = new Set(before.map((e) => `${e.file}|${e.line}|${e.col}|${e.code}`));
const 새오류 = after.filter((e) => !beforeKeys.has(`${e.file}|${e.line}|${e.col}|${e.code}`));
const P = new Set(새오류.map((e) => e.file));

const C = new Set();
for (const f of fs.readdirSync(args.callers)) {
  for (const l of fs.readFileSync(path.join(args.callers, f), 'utf8').split('\n')) {
    const m = l.match(/^\s{2}\S+\s+\S+\s+(\S+):\d+\s*$/);
    if (m) C.add(m[1]);
  }
}
const D = new Set([args.deleted]);

const isDecl = (st) => ts.isFunctionDeclaration(st) || ts.isClassDeclaration(st) || ts.isInterfaceDeclaration(st) ||
  ts.isTypeAliasDeclaration(st) || ts.isEnumDeclaration(st) || ts.isVariableStatement(st) || ts.isModuleDeclaration(st);

const out = [];
const say = (s = '') => out.push(s);
say('# E4 — 깨질 곳의 대조');
say('');
say(`- typescript \`${ts.version}\` · 복제본 \`${path.basename(root)}\``);
say(`- tsc 전 오류 ${before.length} · A 뒤 오류 ${after.length} · 새 오류 ${새오류.length}`);
say(`- P(새 오류 파일) ${[...P].sort().join(' · ') || '없음'}`);
say(`- C(호출자 파일) ${[...C].sort().join(' · ') || '없음'}`);
say(`- D(지운 파일) ${[...D].join(' · ')}`);
const PC = [...P].filter((f) => C.has(f)).sort();
const rest = [...P].filter((f) => !C.has(f) && !D.has(f)).sort();
say(`- **P∩C** ${PC.join(' · ') || '없음'}`);
say(`- **P∖(C∪D)** ${rest.join(' · ') || '없음'}`);
say('');

let 선언안 = 0;
for (const f of rest) {
  const abs = path.join(root, f);
  const sf = ts.createSourceFile(abs, fs.readFileSync(abs, 'utf8'), ts.ScriptTarget.Latest, true);
  say(`## ${f}`);
  for (const e of 새오류.filter((x) => x.file === f)) {
    const pos = sf.getPositionOfLineAndCharacter(e.line - 1, e.col - 1);
    const st = sf.statements.find((s) => s.getFullStart() <= pos && pos < s.getEnd());
    const 종류 = st ? ts.SyntaxKind[st.kind] : '(문장 밖)';
    const 자리 = st && isDecl(st) ? '선언 안' : '최상위';
    if (자리 === '선언 안') 선언안 += 1;
    say(`- ${e.line}:${e.col} ${e.code} · ${종류} · **${자리}** · ${e.msg.slice(0, 120)}`);
  }
  say('');
}
say(`## 판정 — P∖(C∪D) 에서 오류 자리가 선언 안인 것 **${선언안}** (0 이어야 한다)`);
console.log(out.join('\n'));
process.exitCode = 선언안 === 0 ? 0 : 1;
