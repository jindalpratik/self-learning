for i in range(int(input())):
    n = int(input())
    liElements = [int(x) for x in input().split()]
    dictElements = {}
    max_count = 0
    for i in liElements:
        dictElements[i] = dictElements.get(i, 0) + 1
    max = 0
    for i in dictElements:
        if dictElements[i] > max:
            max = dictElements[i]
    print(n-max)
