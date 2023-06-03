import math

for i in range(int(input())):
    [t1, t2, r1, r2] = [int(x) for x in input().split()]
    if (math.pow(t1, 2)/math.pow(r1, 3) == math.pow(t2, 2)/math.pow(r2, 3)):
        print('YES')
    else:
        print('No')
