for i in range(int(input())):
    n = int(input())
    liElements = []
    for i in input().split():
        liElements.append(int(i))
    tot = len(set(liElements))
    print(tot)
