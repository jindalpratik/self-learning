for _ in range(int(input())):
    [X, A, B, C] = [int(x) for x in input().split()]
    li_r = [A,B,C]
    li_r.sort()
    print(li_r[0]*(X-1) + li_r[1])