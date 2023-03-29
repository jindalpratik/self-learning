def gcd(m,n):

    for i in range(min(m,n),0,-1):
        if m%i == 0 and n%i == 0:
            return i


# m = int(input())
# n = int(input())
print(gcd(15584984,19484651))