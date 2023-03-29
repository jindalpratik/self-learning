def isprime(k):
    if k == 1:
        return False
    if k == 2 or k == 3:
        return True
    for i in range(2, k-1):
        if k % i == 0:
            return False
    return True


def primepartition(m):
    if m < 4:
        return False
    if isprime(m-2):
        return True
    l = 2
    m -= 2
    while m > 0:
        if isprime(l) and isprime(m):
            return True
        m -= 1
        l += 1
    return False


def matched(s):
    li = []
    for i in s:
        if i == '(':
            li.append(1)
            continue
        if i == ')':
            if li == []:
                return False
            else:
                li.remove(1)
    if li == []:
        return True
    else:
        return False


def rotatelist(l, k):
    lr = []
    lr.extend(l)
    while k > 0:
        m = lr[0]
        lr = lr[1:]
        lr.append(m)
        k -= 1

    return lr
