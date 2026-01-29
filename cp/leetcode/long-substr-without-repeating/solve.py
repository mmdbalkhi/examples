class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        lengh_1 = 0
        lengh_2 = 0
        long_str = ""

        index = 0
        while index < len(s):
            if s[index] in long_str:
                s = s[s.index(s[index]) + 1 :]
                lengh_2 = lengh_1 if lengh_1 > lengh_2 else lengh_2
                lengh_1 = 0
                index = 0
                long_str = ""
                continue
            long_str += s[index]
            index += 1
            lengh_1 += 1
        return max(lengh_1, lengh_2)
