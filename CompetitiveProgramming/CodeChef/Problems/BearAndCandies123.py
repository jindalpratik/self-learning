for i in range(int(input())):
    [a, b] = [int(x) for x in input().split()]
    tot_candies = 0
    limak_candies = 0
    bob_candies = 0
    turn_l = True
    while True:
        tot_candies += 1
        if turn_l:
            temp = limak_candies + tot_candies
            if temp <= a:
                limak_candies = temp
            else:
                break
            turn_l = False
        else:
            temp = bob_candies + tot_candies
            if temp <= b:
                bob_candies = temp
            else:
                break
            turn_l = True
    if limak_candies < bob_candies:
        print('Bob')
    else:
        print('Limak')
