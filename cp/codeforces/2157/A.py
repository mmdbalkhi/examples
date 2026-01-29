import sys
import typing

input = sys.stdin.readline

t = int(input())
test_cases = []

for _ in range(t):
    size = int(input())
    arr = list(map(int, input().split()))
    test_cases.append(arr)


def solve(nums: typing.List[int]) -> int:
    nums.sort()

    needs_remove = 0
    unique_nums = set(nums)

    for num in unique_nums:
        num_count = nums.count(num)
        if num_count < num:
            needs_remove += num_count
        elif num_count == num:
            continue
        else:
            needs_remove += abs(num - num_count)
    return needs_remove


for test_case in test_cases:
    print(solve(test_case))
