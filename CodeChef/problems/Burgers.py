for i in range(int(input())):
    [a,b] = [int(x) for x in input().split()]
    if a<=b:
        print(a)
    else:
        print(b)