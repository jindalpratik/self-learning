for _ in range(int(input())):
    n = int(input())
    half = n//2
    theStr = input()
    if theStr[:half] == theStr[half:]:
        print('YES')
    else:
        print('NO')
