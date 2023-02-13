n = int(input())
li1 = [int(x) for x in input().split]
set1 = set(li1)
li2 = [int(x) for x in input().split]
set2 = set(li1)
set3 = set1 - set2
print(len(set3))