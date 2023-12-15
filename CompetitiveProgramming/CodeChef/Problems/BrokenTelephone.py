for _ in range(int(input())):
    n = int(input())
    int_arr = [int(x) for x in input().split()]
    count = 0
    if int_arr[0] != int_arr[1]:
        count += 1
    for i in range(1,n-1):
        if int_arr[i] != int_arr[i-1] or int_arr[i] != int_arr[i+1]:
            count += 1
    if int_arr[-1] != int_arr[-2]:
        count += 1
    print(count)