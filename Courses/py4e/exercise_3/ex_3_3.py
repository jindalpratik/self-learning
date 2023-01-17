s_score = input('Enter score; ')

try:
    f_score = float(s_score)
except:
    print("Bad score")
    quit()

if f_score >= 1.0 :
    print("Bad score")
    quit()
elif f_score <= 0.0 :
    print("Bad score")
    quit()
else:
    if f_score >= 0.9 :
        print("A")
    elif f_score >= 0.8 :
        print("B")
    elif f_score >= 0.7 :
        print("C")
    elif f_score >= 0.6 :
        print("D")
    elif f_score < 0.6 :
        print("F")
    else:
        print("Bad score")
        quit()