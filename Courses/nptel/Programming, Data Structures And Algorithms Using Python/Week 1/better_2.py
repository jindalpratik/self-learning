def gcd(m,n):
    
    for i in range(1, min(m,n)+1):
        if m%i == 0 and n%i == 0:
            mrcf = i
    return mrcf

# m = int(input())
# n = int(input())
print(gcd(15584984,19484651))