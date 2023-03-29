def gcd(m,n):
    if m < n:
        (m,n) = (n,m)
        
    while m%n != 0:
        diff = m-n
        (m,n) = (max(n,diff),min(n,diff))
        
    return n
    

# m = int(input())
# n = int(input())
print(gcd(15584984,19484651)) 