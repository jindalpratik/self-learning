def rainaverage(li):
    count = {}
    sum = {}
    average = {}
    for i in li:
        count[i[0]] = count.get(i[0], 0) + 1
        sum[i[0]] = sum.get(i[0], 0) + i[1]
    li1 = list(count.keys())
    li1.sort()
    resultlist = []
    for i in li1:
        resultlist.append((i,(sum[i]/count[i])))
    return resultlist

def listtype(l):
  return(type(l) == type([]))

def flatten(li):
    new_li = []
    for i in li:
        if not listtype(i):
            new_li.append(i)
        else:
            for k in i:
                new_li.append(k)
    for f in new_li:
        if listtype(i):
            new_li = flatten(new_li)
            
    return new_li
