for i in range(int(input())):
    int_arr = [int(x) for x in list(input())]
    count = 0
    for f in int_arr:
        if f == 4:
            count += 1
    print(count)
