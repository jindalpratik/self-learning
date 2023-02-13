n = int(input())
li1 = [int(x) for x in input().split()]
set1 = set(li1)
# print(set1)
n = int(input())
li2 = [int(x) for x in input().split()]
set2 = set(li2)
# print(set2)
set3 = set1.union(set2)
print(len(set3))