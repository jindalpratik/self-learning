for _ in range(int(input())):
    N = int(input())
    li_n = [int(x) for x in input().split()]
    if (N%2 != 0):
        print('NO')
        continue
    dict_n = {}
    for i in li_n:
        dict_n[i] = dict_n.get(i, 0) + 1
    flag = True
    for i in dict_n:
        if(dict_n[i] % 2 != 0):
            flag = False
    if flag:
        print('YES')
    else:
        print('NO')