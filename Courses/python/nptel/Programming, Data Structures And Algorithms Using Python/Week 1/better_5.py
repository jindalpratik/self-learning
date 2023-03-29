def gcd(m,n):
    if m < n:
        (m,n) = (n,m)
        
    if (m%n) == 0:
        return(n)
    else:
        return(gcd(n,m%n))
    
    

# m = int(input())
# n = int(input())
print(gcd(15584984,19484651)) 