#User function Template for python3

class Solution:
    def convertToWave(self, arr, N):
        if N%2 == 0:
            for i in range(0,N,2):
                arr[i] , arr[i+1] = arr[i+1] , arr[i]
            str_1 = ""
            for i in arr:
                str_1 = str(i) + " "
            return str_1
        else:
            for i in range(0,N-1,2):
                arr[i] , arr[i+1] = arr[i+1] , arr[i]
            str_1 = ""
            for i in arr:
                str_1 = str(i) + " "
            return str_1
#{ 
#  Driver Code Starts
#Initial Temhttps://practice.geeksforgeeks.org/contest/the-easiest-ever-coding-challenge-4/problems#plate for Python 3

import math
def main():
    T=int(input())
    while(T>0):
        N = int(input())
        arr = [int(x) for x in input().split()]
        ob = Solution()
        ob.convertToWave(arr,N)
        for i in arr:
            print(i,end=" ")
        print()
        T-=1

if __name__=="__main__":
    main()
# } Driver Code Ends