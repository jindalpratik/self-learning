import re

fname = input('Enter a file name: ')
try:
    fhand = open(fname)
except:
    print('Not a correct file name.')
    exit()
tot_count = 0
for line in fhand:
    matching = re.findall('[0-9]+' , line)
    if len(matching) == 0: continue
    for i in matching:
        tot_count += int(i)

print(tot_count)
    