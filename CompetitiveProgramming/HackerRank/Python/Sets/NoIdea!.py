throw = [int(x) for x in input().split()]
arr_main = [int(x) for x in input().split()]
arr_1 = set([int(x) for x in input().split()])
arr_2 = set([int(x) for x in input().split()])
happiness = 0
for i in arr_main:
    if i in arr_1:
        happiness += 1
    elif i in arr_2:
        happiness -= 1
    else:
        continue
print(happiness)