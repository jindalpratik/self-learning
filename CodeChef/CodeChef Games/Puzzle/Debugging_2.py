# cook your dish here
t=int(input())
for i in range(t):
    [x,y,a,b]= [int(x) for x in (input().split())]
    if (x==a) or (y==b):
        print("YES")
    else:
        print("NO")