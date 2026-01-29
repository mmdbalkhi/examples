def max_cost(n: int):
    return  n * (n * n - n - 1)

for _ in range(int(input())):
    print(max_cost(int(input())))
