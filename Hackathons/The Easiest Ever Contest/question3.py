#User function Template for python3

class Solution:
    def specialArray(self, n, arr):
        sum = 0
        for i in range(n):
            if sum >= arr[i]:
                return 0
            else:
                sum += arr[i]
        return 1


#{ 
#  Driver Code Starts
#Initial Template for Python 3

if __name__ == '__main__':
    tc = int(input())
    while tc > 0:
        n = int(input())
        arr = list(map(int, input().split()))
        ob = Solution()
        ans = ob.specialArray(n, arr)
        print(ans)
        tc -= 1
# } Driver Code Ends