for i in range(int(input())):
    [numCandidates, numQues] = [int(x) for x in input().split()]
    [minCorrect, minPartiallyCorrect] = [int(x) for x in input().split()]
    for i in range(numCandidates):
        liQuestionStatus = list(input())
        dictQuestionStatus = {'F': 0, 'P': 0, 'U': 0}
        for i in liQuestionStatus:
            dictQuestionStatus[i] += 1
        if dictQuestionStatus['F'] >= minCorrect:
            print(1, end='')
        elif dictQuestionStatus['F'] == minCorrect - 1 and dictQuestionStatus['P'] >= minPartiallyCorrect:
            print(1, end='')
        else:
            print(0, end='')
    print()
