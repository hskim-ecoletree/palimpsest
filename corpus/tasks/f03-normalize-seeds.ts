// 정규화 속성 검사(`proptest`)의 씨앗 — **실물에서 떠 왔다.**
//
// 출처: ditto @ aded7ce7f88f. 각 선언 위에 원래 파일을 적었다.
//
// # 왜 씨앗이 실물이어야 하는가 (`[f03.2.pass]` ④)
//
// 무작위 TypeScript 를 지어내면 문법에 맞는 것을 만드는 데 힘이 다 가고 정작
// **변형이 얕아진다** — 구조 분해도 템플릿 리터럴도 타입 파라미터도 안 나온다.
// 그래서 씨앗은 실물이고 **변형만 무작위화한다.**
//
// # 이 파일을 손으로 고치지 않는다
//
// 고치면 그 순간 씨앗이 우리가 통과시키고 싶은 모양으로 수렴한다. 늘릴 때는
// 코퍼스에서 **다시 떠 온다.**

// ── src/core/fs.ts ──────────────────────────────────────────────────────────
export class SchemaValidationError extends Error {
  public readonly path: string;

  constructor(path: string, cause: unknown) {
    super(`schema validation failed for ${path}: ${String(cause)}`, { cause });
    this.name = 'SchemaValidationError';
    this.path = path;
  }
}

// ── src/core/fs.ts ──────────────────────────────────────────────────────────
export function isAtOrAboveHome(rel: string, isAbs: (p: string) => boolean = isAbsolute): boolean {
  return rel === '' || (!rel.startsWith('..') && !isAbs(rel));
}

// ── rebuild/drive/backstop.ts ───────────────────────────────────────────────
export interface BackstopDecision {
  tripped: boolean;
  reasons: string[];
}

// ── rebuild/drive/backstop.ts ───────────────────────────────────────────────
export function evaluateBackstop(
  backstop: Backstop,
  opts: { maxNoProgressRounds: number; maxTurns?: number },
): BackstopDecision {
  const reasons: string[] = [];
  if (backstop.no_progress_rounds >= opts.maxNoProgressRounds) {
    reasons.push(
      `no_progress_rounds ${backstop.no_progress_rounds} >= limit ${opts.maxNoProgressRounds}`,
    );
  }
  if (opts.maxTurns !== undefined && backstop.turns >= opts.maxTurns) {
    reasons.push(`turns ${backstop.turns} >= limit ${opts.maxTurns}`);
  }
  const t = backstop.queue_size_trend;
  const n = t.length;
  if (n >= 3) {
    const [a, b, c] = [t[n - 3]!, t[n - 2]!, t[n - 1]!];
    if (c >= b && b >= a && c > a) {
      reasons.push(`queue_size_trend non-draining: [${a}, ${b}, ${c}]`);
    }
  }
  return { tripped: reasons.length > 0, reasons };
}

// ── src/acg/internal-packages.ts ────────────────────────────────────────────
export async function scanLocalJars(root: string, maxDepth = 6): Promise<string[]> {
  const out: string[] = [];
  async function walk(dir: string, depth: number): Promise<void> {
    if (depth > maxDepth) return;
    let entries: Dirent<string>[];
    try {
      entries = await readdir(dir, { withFileTypes: true });
    } catch {
      return;
    }
    for (const e of entries) {
      const p = join(dir, e.name);
      if (e.isDirectory()) await walk(p, depth + 1);
      else if (e.name.endsWith('.jar')) out.push(p);
    }
  }
  await walk(root, 0);
  return out;
}
