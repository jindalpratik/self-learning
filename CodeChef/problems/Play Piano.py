for _ in range(int(input())):
    strInp = input()
    flag = True
    for i in range(len(strInp)//2):
        if strInp[i*2] == strInp[i*2+1]:
            flag = False
            break
    if flag:
        print('yes')
    else:
        print('no')
