for _ in range(int(input())):
    [K, N] = [int(x) for x in input().split()]
    if N == 0:
        if (N+K) % 4 == 0:
            print('Off')
        else:
            print('On')
    if N == 1:
        if K % 4 == 0:
            print('On')
        else:
            print('Ambiguous')
