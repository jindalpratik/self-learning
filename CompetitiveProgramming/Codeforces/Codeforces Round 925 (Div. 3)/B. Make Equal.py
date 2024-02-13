for _ in range(int(input())):
    x = int(input())
    li = [int(y) for y in input().split()]

    sum = 0
    for i in li:
        sum += i

    length = len(li)
    num = sum/length
    flag = True
    if(sum%length == 0):
        for i in range(length - 1):
            if(li[i] >= num):
                temp = li[i] - num
                li[i] -= num
                li[i+1] += temp
            else:
                print("NO")
                flag = False
                break
        if(flag):
            print("YES")
    else:
        print("NO")