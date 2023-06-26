for _ in range(int(input())):
    [N, A, B] = [int(x) for x in input().split()]
    int_arr = [int(x) for x in input().split()]
    baseProb = 1/N
    prob_A = 0
    prob_B = 0
    for i in range(N):
        if int_arr[i] == A:
            prob_A += baseProb
        if int_arr[i] == B:
            prob_B += baseProb
    print(prob_A*prob_B)
            
        