for i in range(int(input())):
    dictScores = {}
    for i in range(4):
        [Name, Score] = input().split()
        dictScores[Name] = int(Score)
    if(dictScores['RealMadrid'] < dictScores['Malaga'] and dictScores['Barcelona'] > dictScores['Eibar']):
        print('Barcelona')
    else:
        print('RealMadrid')