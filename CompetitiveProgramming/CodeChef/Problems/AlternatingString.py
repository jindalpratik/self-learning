for _ in range(int(input())):
    n = int(input())
    listStr = list(input())
    dictNum = {0: 0, 1: 0}
    for i in listStr:
        dictNum[int(i)] = dictNum.get(int(i), 0) + 1

    if dictNum[0] == dictNum[1]:
        print(dictNum[0] + dictNum[1])
    elif dictNum[0] < dictNum[1]:
        print(dictNum[0]*2 + 1)
    else:
        print(dictNum[1]*2 + 1)
