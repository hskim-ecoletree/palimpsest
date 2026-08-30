# RED 관측 — round approve verify

기준 HEAD `e9a1da762e42d6cf1f4b2424387f8ec1fe668ee6`에서 구현 전에 실행했다.

```text
$ cargo test -p pal-cli --test round_approve_verify
running 18 tests
...
test timeout_output_cap과_descendant는_bounded_cleanup된다 ... FAILED
test 실행된_현재_negative_control없이는_주조건도_met이_아니다 ... FAILED
test approval_record변조와_stale_projected_evidence는_fail_closed다 ... FAILED
test 같은_oracle재실행은_새_current_evidence를_append한다 ... FAILED
test 실행중_oracle이나_projected_tree변화는_evidence없이_폐기된다 ... FAILED
test exit_zero_no_expect와_nonzero_marker는_둘다_unmet이다 ... FAILED
test append실패는_재실행하지_않고_partial_line은_invalid다 ... FAILED
test 미승인_oracle과_변경된_path_cwd_shell_budget은_spawn전에_거부된다 ... FAILED

test result: FAILED. 10 passed; 8 failed; 0 ignored; 0 measured; 0 filtered out
error: test failed, to rerun pass `-p pal-cli --test round_approve_verify`
```

실패 원인은 `pal round approve`와 `pal round verify`가 없어 clap이 exit 2를 냈기 때문이다.
미승인 fixture는 요구한 exit 3 대신 exit 2였고, 나머지 공격은 approve 단계부터 진행하지
못했다. 기존 공용 helper 시험 9개와 비실행 `process_helper` 하나는 통과했으므로 모집단이
0인 거짓 RED가 아니다.

## 음성 대조

`실행된_현재_negative_control없이는_주조건도_met이_아니다`는 positive 실행 뒤 control
미실행·실패 상태에서 주 조건이 `met`이 아니어야 하고, control oracle을 실제 성공 실행한
뒤에만 둘과 aggregate가 `met`이어야 한다. 현재는 approve subcommand 부재로 이 장면 자체가
빨갛다. 구현 뒤 같은 시험이 전이를 끝까지 통과해야 한다.
