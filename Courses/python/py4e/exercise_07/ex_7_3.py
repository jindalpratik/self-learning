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
    count+=1
print(count)
