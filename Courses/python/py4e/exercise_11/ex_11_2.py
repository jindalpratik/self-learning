import re

fname = input('Enter a file name: ')
try:
    fhand = open(fname)
except:
    print('Not a correct file name.')
    exit()
count_li = []
for line in fhand:
    matching = re.findall('^New Revision: ([1-9]*)' , line)
    if len(matching) == 0: continue
    count_li.extend(matching)

tot_count = 0
for i in count_li:
    tot_count += int(i)
avg = tot_count//len(count_li)

print(avg)
    