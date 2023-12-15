#User function Template for python3

class Solution:
    def validString(self, N, S):
        smallest = "a"
        # print(S)
        if S == "":
            return 1
        for i in S:
            # print(i , smallest)
            if i < smallest:
                return 0
            else:
                smallest = i
                continue
        return 1
        # code here

        # code here

#{ 
#  Driver Code Starts
#Initial Template for Python 3

if __name__ == '__main__':
    tc = int(input())
    while tc > 0:
        N, S = list(map(str, input().split()))
        N = int(N)
        ob = Solution()
        ans = ob.validString(N, S)
        print(ans)
        tc -= 1
# } Driver Code Ends