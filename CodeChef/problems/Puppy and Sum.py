for _ in range(int(input())):
    D, N = map(int, input().split())
    for i in range(D):
        sum = 0
        for k in range(1, N+1):
            sum += k
        N = sum
    print(N)
