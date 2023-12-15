
class Solution:
    def maximumNumber(self, s, k):
        list_1 = list(s)
        print(list_1)
        for i in range(k):
            for f in range(i,len(list_1)):
                if int(list_1[f]) == 1:
                    continue
                else:
                    for k in range(f+1,len(list_1)):
                        if int(list_1[k]) == 0:
                            continue
                        else:
                            list_1[f], list_1[k] = list_1[k], list_1[f]
                            break
                break
        for i in list_1:
            print(i,end="")