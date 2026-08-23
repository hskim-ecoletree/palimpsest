"""★ O1 의 분모 — 「그 라운드까지 공개된 스펙」. **사전 등록물이다.**"""

공개 = {
    "A": {1: [f"A{i:02d}" for i in range(1, 21)],
          2: [f"A{i:02d}" for i in range(1, 25)],
          4: [f"A{i:02d}" for i in range(1, 29)]},
    "B": {1: [f"B{i:02d}" for i in range(1, 21)],
          2: [f"B{i:02d}" for i in range(1, 25)],
          4: [f"B{i:02d}" for i in range(1, 29)]},
}


def 분모(과제, 라운드):
    """라운드 n 에서 O1 의 분모가 되는 스펙 항목 목록."""
    표 = 공개[과제]
    쓸 = max(k for k in 표 if k <= 라운드)
    return 표[쓸]
