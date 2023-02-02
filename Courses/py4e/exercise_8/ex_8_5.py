fname = input('Enter a file name: ')
try:
    fhand = open(fname)
except:
    if not fname == "na na boo boo":
        print('File cannot be opened:', fname)
        exit()
    else:
        print("NA NA BOO BOO TO YOU - You have been punk'd!")
        exit()
count = 0
for line in fhand:
    line = line.rstrip()
    if not line.startswith('From '):
        continue
    count += 1
    temp = line.split()
    print(temp[1])
print("There were",count,"lines in the file with From as the first word")  
    