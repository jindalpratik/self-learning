for i in range(int(input())):
    [A, B] = [int(x) for x in input().split()]
    Sum = A + B
    dictRequiredMatches = {'0': 6, '1': 2, '2': 5, '3': 5,
                           '4': 4, '5': 5, '6': 6, '7': 3, '8': 7, '9': 6}
    matchesRequired = 0
    for i in str(Sum):
        matchesRequired += dictRequiredMatches[i]
    print(matchesRequired)
