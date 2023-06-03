for i in range(int(input())):
    [a, b, c, d] = [int(x) for x in input().split()]
    if (b > a):
        a += c
    else:
        b += c
    if (b > a):
        a += d
    else:
        b += d
    if (b > a):
        print('S')
    else:
        print('N')
