def computegrade(score):
    # print("in computegrade",score)
    if score >= 1.0 :
        print("Bad score")
        quit()
    elif score <= 0.0 :
        print("Bad score")
        quit()
    else:
        if score >= 0.9 :
            print("A")
        elif score >= 0.8 :
            print("B")
        elif score >= 0.7 :
            print("C")
        elif score >= 0.6 :
            print("D")
        elif score < 0.6 :
            print("F")
        else:
            print("Bad score")
            quit()

s_score = input('Enter score: ')

try:
    f_score = float(s_score)
except:
    print("Bad score")
    quit()

computegrade(f_score)