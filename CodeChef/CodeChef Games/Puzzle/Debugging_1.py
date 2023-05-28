# cook your dish here
for i in range(int(input())):
    A,B=list(map(int,input().split()))

    if (B/A)%2 == 1 or (B/A)%2 == 0:
        print((B//A)-1)
    else:
        print((B//A))
