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
    colpos = li[1].find('@')
    email = li[1]
    di[email[colpos + 1 : ]] = di.get(email[colpos + 1 : ], 1) + 1
print(di)
# max = 0
# index = 0
# for i in di:
#     if di[i] > max:
#         index = i
#         max = di[i]
# print(index , di[index])