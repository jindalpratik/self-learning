fname = input('Enter a file name: ').strip('"')
try:
    fhand = open(fname)
except:
    print('Not a correct file name.')
    exit()
line_till_point = ""
count = 0
count_2 = 0
prev = ""
for line in fhand:
    # print(line)
    count += 1
    line_till_point += line
    word_list_line = line.split()
    for word in word_list_line:
        # print(word)
        if "Rs" in word:
            # print(word)
            cost = word.split("Rs_")
            # print(cost)
            if float(cost[1]) < 150:
                count_2 += 1
                print(prev,"\n",cost[1], count)
    prev = line
print(count_2)
