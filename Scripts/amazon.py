fname = input('Enter a file name: ')
try:
    fhand = open(fname)
except:
    print('Not a correct file name.')
    exit()
line_till_point = ""
count = 0
count_2 = 0
for line in fhand:
    count += 1
    line_till_point += line
    word_list_line = line.split()
    for word in word_list_line:
        if "Rs." in word:
            cost = word.split(".")
            if int(cost[1]) < 150:
                count_2 += 1
                print(word, count)

print(count_2)
