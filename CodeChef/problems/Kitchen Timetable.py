for i in range(int(input())):
    n = int(input())
    a_arr = [int(x) for x in input().split()]
    b_arr = [int(x) for x in input().split()]
    a_arr.insert(0, 0)
    count = 0
    for k in range(n):
        if (a_arr[k+1]-a_arr[k]) >= b_arr[k]:
            count += 1
    print(count)
