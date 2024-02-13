for _ in range(int(input())):
    num = int(input())

    tot = 3
    coutz = 0
    if(num == 27 or num == 28):
        #Cause couldn't be bothered
        k = 0
    else:
        while(num > 26):
            num -= 26
            coutz += 1

    if((tot-coutz) == 3):
        print("aa",end="")
        print(chr(ord('a')+num-3))
    elif((tot-coutz) == 2):
        print("a",end="")
        print(chr(ord('a')+num-2),end="")
        print("z")
    elif((tot-coutz) == 1):
        print(chr(ord('a')+num-1),end="")
        print("z",end="")
        print("z")
        
        