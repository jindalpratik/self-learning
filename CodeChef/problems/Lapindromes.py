for _ in range(int(input())):
    inpStr = input()
    Odd = False
    if len(inpStr) % 2 != 0:
        Odd = True
        half = len(inpStr)//2 + 1
    else:
        half = len(inpStr)//2
    firstStr = inpStr[:half]
    if Odd:
        secondStr = inpStr[half - 1:]
    else:
        secondStr = inpStr[half:]
    dictStr = {}
    for i in firstStr:
        dictStr[i] = dictStr.get(i, 0) + 1
    for i in secondStr:
        dictStr[i] = dictStr.get(i, 0) - 1
    flag = True
    for i in dictStr:
        if dictStr[i] != 0:
            flag = False
    if flag:
        print('YES')
    else:
        print('NO')
