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
    di[li[1]] = di.get(li[1], 0) + 1
li_1 = []
for i in di:
    li_1.append((di[i],i))
li_1.sort(reverse=True)
print(li_1[0][1],li_1[0][0])