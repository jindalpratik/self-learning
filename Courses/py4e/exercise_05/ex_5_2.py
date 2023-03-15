largest = None
smallest = None
while True:
    snum = input("Enter a number: ")
    #print(snum)
    if snum == "done":
        break
    try:
        inum = int(snum)
        #print(inum)
    except:
        print('Invalid input')
        continue
    if largest is None:
        largest = inum
        #print(largest, inum)
    elif largest < inum:
        largest = inum
        #print(largest, inum)
    if smallest is None:
        smallest = inum
        #print(smallest, inum)
    elif smallest > inum:
        smallest = inum
        #print(smallest, inum)
print("Maximum is", largest)
print("Minimum is", smallest)

#Um yeah forgot about the int variable for a second and confused it with var for some reason.