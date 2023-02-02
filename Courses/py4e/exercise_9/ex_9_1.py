fhand = open('words.txt')
dicti = {}
for line in fhand:
    for i in line:
        dicti[i] = dicti.get(i,1) + 1
print(dicti)