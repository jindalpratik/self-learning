for i in range(int(input())):
    [N,K] = [int(x) for x in input().split()]
    int_arr = [int(x) for x in input().split()]
    count = 0
    for m in int_arr:
        if m > K:
            count += 1
    print(count)