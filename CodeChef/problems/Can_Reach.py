for _ in range(int(input())):
    [x, y, K] = [int(x) for x in input().split()]
    if x % K == 0 and y % K == 0:
        print('YES')
    else:
        print('NO')
