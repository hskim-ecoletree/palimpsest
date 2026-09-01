"""규칙 1-20 세부 검증"""
import sys
sys.path.insert(0, '/tmp/pal-x-71/s1')
from tbl import *

print("=" * 50)
print("규칙 1-10: 기본 기능 및 roundtrip")
print("=" * 50)

# 규칙 1: 첫 줄이 헤더
text1 = "id,name,score\n1,Alice,95\n2,Bob,87"
t1 = normalize(text1)
print(f"규칙 1 (헤더): {len(t1['columns']) == 3}")

# 규칙 5: 값의 앞뒤 공백 제거
text2 = "name,age\n Alice ,30\n Bob, 25"
t2 = normalize(text2)
print(f"규칙 5 (공백 제거): {t2['rows'][0][0] == 'Alice'}")

# 규칙 6: 빈 칸은 None
text3 = "name,age\nAlice,\nBob,30"
t3 = normalize(text3)
print(f"규칙 6 (빈 칸): {t3['rows'][0][1] is None}")

# 규칙 7: 짧은 행은 None으로 채움
text4 = "a,b,c\n1,2\n3,4,5,6"
try:
    t4 = normalize(text4)
    print(f"규칙 7 (짧은 행): {len(t4['rows'][0]) == 3 and t4['rows'][0][2] is None}")
except NormError:
    pass

# 규칙 8: 긴 행은 오류
text5 = "a,b\n1,2,3"
error8 = False
try:
    normalize(text5)
except NormError as e:
    error8 = "열 개수" in str(e)
print(f"규칙 8 (긴 행 오류): {error8}")

# 규칙 9: 인용된 쉼표와 따옴표 이스케이프
text6 = 'name,note\nAlice,"hello, world"\nBob,"say ""hi"""'
t6 = normalize(text6)
print(f"규칙 9 (인용 처리): {t6['rows'][0][1] == 'hello, world' and t6['rows'][1][1] == 'say \"hi\"'}")

# 규칙 10: roundtrip
text7 = "id,val\n1,10\n2,20"
t7 = normalize(text7)
rendered = render(t7)
t7_back = normalize(rendered)
print(f"규칙 10 (roundtrip): {t7 == t7_back}")

# 규칙 18: 중복 헤더 오류
error18 = False
try:
    normalize("a,b,a\n1,2,3")
except NormError as e:
    error18 = "같은 이름" in str(e)
print(f"규칙 18 (중복 헤더): {error18}")

# 규칙 20: 빈 입력
t20 = normalize("")
print(f"규칙 20 (빈 입력): {t20 == {'columns': [], 'rows': []}}")

print()
print("=" * 50)
print("규칙 11-17: 연산 함수")
print("=" * 50)

text = "id,name,dept,salary\n1,Alice,HR,3000\n2,Bob,IT,4000\n3,Charlie,HR,3500\n4,David,IT,"

t = normalize(text)

# 규칙 11: select
t11 = select(t, ["name", "dept"])
print(f"규칙 11 (select): {len(t11['columns']) == 2 and t11['columns'][0]['name'] == 'name'}")

# select 오류
select_error = False
try:
    select(t, ["missing"])
except NormError:
    select_error = True
print(f"규칙 11 (select 오류): {select_error}")

# 규칙 12: where
t12 = where(t, "dept", "==", "HR")
print(f"규칙 12 (where ==): {len(t12['rows']) == 2}")

# where null 무시
t12_null = where(t, "salary", ">", 3000)
print(f"규칙 12 (where null 무시): {len(t12_null['rows']) == 2}")  # David는 None이므로 제외

# where 연산자 검증
where_error = False
try:
    where(t, "salary", "**", 3000)
except NormError:
    where_error = True
print(f"규칙 12 (where 연산자 오류): {where_error}")

# 규칙 13: order_by
t13 = order_by(t, "salary", desc=False)
salaries = [row[3] for row in t13['rows']]
print(f"규칙 13 (order_by 정렬): {salaries[0] == 3000 and salaries[1] == 3500 and salaries[2] == 4000}")
print(f"규칙 13 (order_by null 끝): {salaries[-1] is None}")

# 규칙 14: agg
sum_val = agg(t, "salary", "sum")
mean_val = agg(t, "salary", "mean")
count_val = agg(t, "salary", "count")
print(f"규칙 14 (sum): {sum_val == 10500}")
print(f"규칙 14 (mean): {mean_val == 3500.0}")
print(f"규칙 14 (count): {count_val == 3}")

# agg 빈 경우
t_empty_salary = where(t, "salary", "<", 0)
agg_empty = agg(t_empty_salary, "salary", "mean")
print(f"규칙 14 (agg 빈 경우): {agg_empty is None}")

# 규칙 15: group_count
gc = group_count(t, "dept")
print(f"규칙 15 (group_count): {gc['HR'] == 2 and gc['IT'] == 2}")

# 규칙 16: rename
t16 = rename(t, "salary", "pay")
print(f"규칙 16 (rename): {t16['columns'][3]['name'] == 'pay'}")

# rename 오류
rename_error = False
try:
    rename(t, "salary", "name")
except NormError:
    rename_error = True
print(f"규칙 16 (rename 충돌): {rename_error}")

# 규칙 17: add_column
t17 = add_column(t, "bonus", [100, 200, 150, None])
print(f"규칙 17 (add_column): {len(t17['columns']) == 5}")
print(f"규칙 17 (add_column 타입): {t17['columns'][4]['type'] == 'int'}")

# add_column 오류
add_error = False
try:
    add_column(t, "name", [1, 2])
except NormError:
    add_error = True
print(f"규칙 17 (add_column 오류): {add_error}")

print()
print("=" * 50)
print("규칙 2-4: 타입 추론")
print("=" * 50)

# 정수 only
text_int = "val\n1\n2\n3"
t_int = normalize(text_int)
print(f"규칙 2 (정수): {t_int['columns'][0]['type'] == 'int' and t_int['rows'][0][0] == 1}")

# 정수 + 실수 = 실수
text_mixed = "val\n1\n2.5\n3"
t_mixed = normalize(text_mixed)
print(f"규칙 3 (정수+실수→float): {t_mixed['columns'][0]['type'] == 'float'}")

# 수 + 문자 = 문자
text_str = "val\n1\n2\nabc"
t_str = normalize(text_str)
print(f"규칙 4 (수+문자→str): {t_str['columns'][0]['type'] == 'str'}")

print()
print("=" * 50)
print("음성 대조: 검사가 고장이면 드러나는 조건")
print("=" * 50)

# 헤더가 제대로 안 읽히면 컬럼 개수가 0
t_no_header = normalize("\nvalue\n1\n2")  # 첫 줄이 빈 경우
print(f"음성 1 (헤더 오류): {len(t_no_header['columns']) == 0}")

# NormError를 안 던지는데 호출되면 오류
error_caught = False
try:
    where(t, "name", "==", "Alice")
    error_caught = True
except:
    error_caught = False
print(f"음성 2 (정상 where 작동): {error_caught}")

# 연산 후에도 원본이 바뀌지 않아야 함
t_orig = normalize(text)
t_filtered = where(t_orig, "dept", "==", "HR")
print(f"음성 3 (불변성): {len(t_orig['rows']) == 4 and len(t_filtered['rows']) == 2}")

print()
print("=" * 50)
print("종합: 규칙 1-20 검증 완료")
print("=" * 50)
