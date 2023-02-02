li = []
while True:
    inp1 = input("Enter a number: ")
    if inp1 == 'done':
        break
    try:
        inp = int(inp1)
    except:
        print("Not a number")
        continue
    li.append(inp)
print("Maxinmum:",max(li))
print("Minimum:",min(li))