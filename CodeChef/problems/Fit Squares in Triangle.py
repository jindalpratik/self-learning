for _ in range(int(input())):
    n = int(input())
    if n < 4:
        print('0')
        continue
    n = n - 2
    n = n//2
    print(n*(n+1)//2)
