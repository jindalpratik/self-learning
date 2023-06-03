for i in range(int(input())):
    n = int(input())
    X = [int(x) for x in input().split()]
    Y = [int(x) for x in input().split()]
    count = 0
    maxCount = 0
    for k in range(n):
        if (X[k] > 0 and Y[k] > 0):
            count += 1
            if count > maxCount:
                maxCount = count
        else:
            count = 0
    print(maxCount)
