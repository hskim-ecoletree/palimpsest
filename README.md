# palimpsest (`pal`)

코드를 고치려는 순간, **그 코드에 걸린 결정이 아직 유효한지**와 **바꾸면 어디가 닿는지**를 알려 주는 도구입니다.

프로젝트의 결정·설계 문서(ADR 등)를 코드 좌표에 결박해 두고, `pal touch <심볼>` 한 번으로
그 좌표에 걸린 결정 · 신선도(`fresh` / `stale`) · 호출자 자리 · 못 푼 참조 · 답의 근거를 받습니다.
모르는 것은 모른다고 적습니다 — 「이상 없음」이라고 말하지 않습니다.

- **지원 언어**: TypeScript · Rust (첫 릴리스). 그 밖의 파일은 대장에 「결박 불가」로 적힙니다.
- **지원 플랫폼**: Linux x86_64 · macOS (Apple Silicon · Intel) · Windows x86_64
- **필요한 것**: `git` 이 `PATH` 에 있어야 합니다. 대상 프로젝트를 빌드하지 않습니다.

## 1. 받기

[릴리스 페이지](https://github.com/hskim-ecoletree/palimpsest/releases/latest)에서 플랫폼에 맞는 자산 하나를 받습니다.

| 플랫폼 | 자산 |
|---|---|
| Linux x86_64 | `pal-x86_64-unknown-linux-gnu.tar.gz` |
| macOS Apple Silicon | `pal-aarch64-apple-darwin.tar.gz` |
| macOS Intel | `pal-x86_64-apple-darwin.tar.gz` |
| Windows x86_64 | `pal-x86_64-pc-windows-msvc.zip` |

같이 올라온 `SHA256SUMS` 로 대조하십시오(서명은 없습니다). 풀어서 나온 `pal`(Windows 는 `pal.exe`)을
`PATH` 위의 디렉터리에 둡니다. **이름을 바꾸지 마십시오** — 훅 등록이 `PATH` 의 이름 `pal` 을 가리킵니다.

```bash
tar xzf pal-aarch64-apple-darwin.tar.gz
install -m 0755 pal-aarch64-apple-darwin/pal ~/.local/bin/pal
pal --version
```

## 2. 프로젝트에 놓기

프로젝트 루트(git 저장소)에서:

```bash
pal install
pal doctor --install
```

`pal install` 은 그 프로젝트 안에만 씁니다 — `.claude/` 아래의 훅·스킬과 `.palimpsest/manifest.toml`
(저장소 식별자 선언)입니다. **`.palimpsest/manifest.toml` 은 커밋하십시오** — 팀원이 다른 디렉터리 이름으로
클론해도 같은 결박을 받게 하는 선언입니다. 걷어낼 때는 `pal uninstall` 입니다.

## 3. 문서를 코드에 걸기

```bash
pal narrative
```

문서 조각을 코드 좌표에 대 보고 **결박됨 · 후보 · 미결박** 셋으로 나눕니다. 아무것도 스스로 승인하지 않습니다.

## 4. 코드를 만지기 전에

```bash
pal touch <심볼 이름>
```

화면의 절:

| 절 | 무엇 |
|---|---|
| `■ 이 좌표에 걸린 것` | 승인된 결정과 그 신선도 — 결박 뒤 감시 대상이 바뀌었으면 `stale` |
| `■ 승인 대기` | 이 좌표를 가리키는 문서 조각. 화면이 **그대로 칠 승인 명령**을 싣습니다 |
| `■ 이 심볼이 하는 것` | 호출자 수와 자리(`경로:줄`). 많으면 나머지를 펴는 명령을 싣습니다. 호출자 수는 하한입니다 |
| `■ 내가 모르는 것` | 못 푼 참조와 그 까닭 |
| `■ 이 답의 근거` | 어느 스냅샷 · 몇 파일을 봤고 무엇을 못 봤는지 |

같은 이름의 심볼이 여럿이면 후보와 지목 문자열을 보여 주고 고르지 않습니다 — `--pick <짧은 해시>` 로 지목합니다.

승인 대기 조각을 승인하면 결박이 생깁니다:

```bash
pal narrative --approve <개체> --pick <짧은 해시>   # touch 화면이 싣는 줄 그대로
pal touch <심볼 이름>                                # ■ 이 좌표에 걸린 것 (1)
```

결박은 `.palimpsest/intent/bindings.jsonl` 에 덧붙여집니다. 팀과 나누려면 커밋하고, 받은 쪽은
`pal intent import .palimpsest/intent/bindings.jsonl` 로 들입니다.

## 라이선스

[MIT](LICENSE-MIT) 또는 [Apache-2.0](LICENSE-APACHE) 중 하나를 골라 쓰십시오.
