fname = input('Enter a file name: ')
try:
    fhand = open(fname)
except:
    print('Not a correct file name.')
    exit()
di = {}
for line in fhand:
    line = line.rstrip()
    if not line.startswith('From '):
        continue
    li = line.split()
    li_2 = li[5].split(":")
    di[li_2[0]] = di.get(li_2[0], 0) + 1
# print(di)
li_1 = []
for i in di:
    li_1.append((i,di[i]))
li_1.sort()
for i in li_1:
    print(i[0],i[1])