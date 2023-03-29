fhand = open('romeo.txt')
li = []
for line in fhand:
    temp = line.split()
    for i in temp:
        li.append(i)
set1 = set(li)
li = list(set1)
li.sort()
print(li)