import re

fhand = open("mbox.txt")
usr_input = input("Enter a regular expression: ")
count = 0
for line in fhand:
    matching = re.findall(usr_input , line)
    if len(matching) == 0: continue
    count += 1

print("mbox.txt had",count, "lines that matched the input",usr_input)
    