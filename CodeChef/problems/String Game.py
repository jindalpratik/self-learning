for _ in range(int(input())):
    n = int(input())
    listInput = list(input())
    if n % 2 != 0:
        print('NO')
        continue
    dictChars = {}
    for i in listInput:
        dictChars[i] = dictChars.get(i, 0) + 1
    flag = True
    for i in dictChars:
        if dictChars[i] % 2 == 0:
            continue
        else:
            flag = False
            break
    if flag:
        print('YES')
    else:
        print('NO')
