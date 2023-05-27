for i in range(int(input())):
    [a,b] = [int(x) for x in input().split()]
    if ((a*10) > (b * 5)):
        print('FIRST')
    elif ((a*10) < (b * 5)):
        print('SECOND')
    else:
        print('ANY')