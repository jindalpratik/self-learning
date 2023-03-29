fname = input('Enter a file name: ')
try:
    fhand = open(fname)
except:
    print('File cannot be opened:', fname)
    exit()
count = 0
tot = 0
for line in fhand:
    line = line.rstrip()
    if not line.startswith('X-DSPAM-Confidence:'):
        continue
    count += 1
    tot += float(line[20:])
print("Average spam confidence:", tot/count)
