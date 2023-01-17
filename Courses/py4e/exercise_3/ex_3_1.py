ph = input('Enter Hours: ')
pr = input('Enter Rate: ')
pt = float(ph) * float(pr)
if float(ph) > 40 :
    pt = 40 * float(pr)
    ph = float(ph) - 40
    pr = float(pr) * 1.5
    pe = float(ph) * float(pr)
    pt = pt + pe
print(pt)