"""규칙 21-28 검증 스크립트"""
import sys
sys.path.insert(0, '/tmp/pal-x-71/s1')
from tbl import *
import json
import datetime

print("=" * 50)
print("규칙 21: bool 타입 (true/false 대소문자 무관)")
print("=" * 50)

# 테스트 1: bool 타입 추론
text = "name,active\nAlice,true\nBob,FALSE\nCharlie,True"
t = normalize(text)
print(f"입력: {repr(text)}")
print(f"columns: {t['columns']}")
print(f"rows: {t['rows']}")
print(f"bool 타입 확인: {t['columns'][1]['type'] == 'bool'}")

# 테스트 2: render roundtrip
rendered = render(t)
t2 = normalize(rendered)
print(f"roundtrip 동일: {t == t2}")
print()

print("=" * 50)
print("규칙 22: 모든 값이 결측인 열은 'str'")
print("=" * 50)

text = "name,notes\nAlice,\nBob,\nCharlie,"
t = normalize(text)
print(f"입력: {repr(text)}")
print(f"columns: {t['columns']}")
print(f"notes 열 타입: {t['columns'][1]['type']}")
print(f"'str' 확인: {t['columns'][1]['type'] == 'str'}")
print()

print("=" * 50)
print("규칙 23: to_json 함수")
print("=" * 50)

text = "name,age\nAlice,30\nBob,"
t = normalize(text)
json_str = to_json(t)
print(f"JSON: {json_str}")
parsed = json.loads(json_str)
print(f"파싱된 JSON: {parsed}")
print(f"첫 행 name: {parsed[0]['name']}")
print(f"두 번째 행 age가 null: {parsed[1]['age'] is None}")
print()

print("=" * 50)
print("규칙 24: join 함수")
print("=" * 50)

# 표 a
text_a = "id,name\n1,Alice\n2,Bob"
t_a = normalize(text_a)
print(f"표 a:\n{render(t_a)}")

# 표 b
text_b = "id,city\n1,Seoul\n2,Busan"
t_b = normalize(text_b)
print(f"표 b:\n{render(t_b)}")

# join
result = join(t_a, t_b, "id")
print(f"join 결과:\n{render(result)}")
print(f"결과 행: {result['rows']}")
print()

print("=" * 50)
print("규칙 25: 날짜 타입 (YYYY-MM-DD → datetime.date)")
print("=" * 50)

text = "name,birthdate\nAlice,2000-01-15\nBob,1995-12-25"
t = normalize(text)
print(f"입력: {repr(text)}")
print(f"columns: {t['columns']}")
print(f"타입이 'date': {t['columns'][1]['type'] == 'date'}")
print(f"첫 행의 birthdate: {t['rows'][0][1]}")
print(f"타입이 datetime.date: {isinstance(t['rows'][0][1], datetime.date)}")

# render로 다시 변환
rendered = render(t)
print(f"렌더링 결과:\n{rendered}")
t2 = normalize(rendered)
print(f"roundtrip 동일: {t == t2}")
print()

print("=" * 50)
print("규칙 26: missing_report 함수")
print("=" * 50)

text = "name,age,city\nAlice,30,Seoul\nBob,,Busan\nCharlie,35,"
t = normalize(text)
report = missing_report(t)
print(f"missing_report: {report}")
print(f"age 열 결측 비율: {report['age']}")
print(f"city 열 결측 비율: {report['city']}")
print()

print("=" * 50)
print("규칙 27: pivot_count 함수")
print("=" * 50)

text = "category,status\nA,pass\nA,pass\nA,fail\nB,pass\nB,fail\nB,fail"
t = normalize(text)
result = pivot_count(t, "category", "status")
print(f"입력:\n{render(t)}")
print(f"pivot_count 결과: {result}")
print()

print("=" * 50)
print("규칙 28: 만 행짜리 입력에서도 roundtrip")
print("=" * 50)

# 10,000 행 생성
lines = ["id,value"]
for i in range(10000):
    lines.append(f"{i},value_{i}")
text = "\n".join(lines)

t = normalize(text)
rendered = render(t)
t2 = normalize(rendered)

print(f"입력 행 수: {len(t['rows'])}")
print(f"roundtrip 후 행 수: {len(t2['rows'])}")
print(f"columns 동일: {t['columns'] == t2['columns']}")
print(f"첫 행 동일: {t['rows'][0] == t2['rows'][0]}")
print(f"마지막 행 동일: {t['rows'][-1] == t2['rows'][-1]}")
print(f"전체 행 동일: {t['rows'] == t2['rows']}")
print()

print("=" * 50)
print("종합: 모든 규칙 검증 완료")
print("=" * 50)
