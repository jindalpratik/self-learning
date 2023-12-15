#{ 
#Driver Code Starts
#Initial Template for Python 3

 # } Driver Code Ends
#User function Template for python3
class Solution:
    def countSubstring(self, S): 
        N = len(S)
        a = 0
        z = {}
        y = 0
        for i in range(N):
            
            if (S[i] >= 'A' and S[i] <= 'Z'):
                y += 1
            else:
                y -= 1
    
            if (y == 0):
                a += 1
    
            if (y in z):
                a += (z[y])
    
            if y in z:
                z[y] += 1
            else:
                z[y] = 1
    
        return a

#{ 
#Driver Code Starts.
if __name__ == '__main__': 
    t = int(input())
    for _ in range(t):
        S = input()
        ob = Solution()
        ans = ob.countSubstring(S)
        print(ans)

#} Driver Code Ends