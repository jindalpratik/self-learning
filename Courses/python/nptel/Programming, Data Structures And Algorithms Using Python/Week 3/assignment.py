def remdup(l):
    seen = set()
    for i in reversed(l):
        if i in seen:
            l.remove(i)
        else:
            seen.add(i)
    return l

def splitsum(li):
    pos = 0
    neg = 0
    for i in li:
        if i < 0:
            neg += i**3
        else:
            pos += i**2
    li = [pos , neg]
    return li
        
def matrixflip(m, d):

    if d == 'h':
        new_m = []
        for i in m:
            temp = []
            for l in i:
                temp.insert(0, l)
            new_m.append(temp)
        return new_m
    elif d == 'v':
        new_m = []
        for i in m:
            new_m.insert(0, i)
        return new_m
    else:
        print("here")
        return m
