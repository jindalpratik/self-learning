for _ in range(int(input())):
    S = input()
    liS = []
    count = 0
    for i in range(0,len(S)-1):
        if 'A' <= S[i] <= 'Z' and 'A' <= S[i+1] <= 'Z':
            tempStr = S[i] + S[i+1]
            if tempStr not in liS:
                liS.append(tempStr)
                count += 1
    print(count)