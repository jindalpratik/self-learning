li = []
while True:
    inp = input("Enter a number: ")
    if inp == 'done':
        break
    li.append(inp)
print("Maxinmum:",max(li))
print("Minimum:",min(li))