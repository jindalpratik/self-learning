#{ 
#Driver Code Starts
#Initial Template for Python 3

 # } Driver Code Ends
#User function Template for python3

class Solution:
    def totalWays(self, n):
        if n==0 or n==1:
            return 1
        else:
            return n * ob.totalWays(n - 1)

#{ 
#Driver Code Starts.

if __name__ == '__main__':
    tc = int(input())
    while tc > 0:
        n = int(input())
        ob = Solution()
        ans = ob.totalWays(n)
        print(ans)
        tc -= 1
#} Driver Code Ends