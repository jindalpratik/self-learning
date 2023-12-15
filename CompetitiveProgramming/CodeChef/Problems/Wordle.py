for _ in range(int(input())):
    firstStr = input()
    secondStr = input()
    for i in range(5):
        if firstStr[i] == secondStr[i]:
            print('G', end = '')
        else:
            print('B', end = '')
    print()