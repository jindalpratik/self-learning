[N, M] = [int(x) for x in input().split()]
str_array = []
for _ in range(N):
    str_array.append(input())
for _ in range(M):
    flag = False
    temp = input()
    if len(temp) >= 47:
        flag = True
    else:
        for i in range(N):
            if str_array[i] in temp:
                flag = True
                break
    if flag:
        print('Good')
    else:
        print('Bad')