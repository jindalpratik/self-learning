for i in range(0,int(input())):
    int_arr = [int(x) for x in input().split()]
    if int_arr[0] > int_arr[1]*10: print("YES")
    else: print("NO")