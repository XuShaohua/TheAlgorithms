
class Solution:
    # Reverse string
    def isPalindrome(self, s: str) -> bool:
        pass

    # Two pointers
    def isPalindrome2(self, s: str) -> bool:
        s_len = len(s)
        left = 0
        right = s_len - 1
        while left < right:
            while left < right and not s[left].isalnum():
                left += 1
            while left < right and not s[right].isalnum():
                right -= 1
            if s[left].lower() != s[right].lower():
                return False
            left += 1
            right -= 1
        return True
