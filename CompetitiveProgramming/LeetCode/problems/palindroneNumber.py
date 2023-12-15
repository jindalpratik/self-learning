class Solution:
    def isPalindrome(self, x: int) -> bool:
        x = str(x)  # type: ignore
        if (x == x[::-1]): #type: ignore
            return True
        else:
            return False
