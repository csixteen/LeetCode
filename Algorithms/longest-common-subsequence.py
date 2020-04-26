#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/longest-common-subsequence/

import unittest


class Solution:
    def longestCommonSubsequence(self, text1: str, text2: str) -> int:
        """ Solution with memoization. It's accepted, but it's
        pretty slow.
        The basic idea is:
        1-) if the first character of each string is the same (better yet,
        the character on indexes `i1` and `i2`), then we'll add 1 to
        the result of computing the longest common subsequence on the
        remainder of both strings.
        2-) if the first characters differ, then we'll calculate what is
        the longest common subsequence between the first string without
        its first char and the second string AND the longest common
        subsequence between the first string and the second string without
        its first char. We'll then return the maximum between these two
        values.

        Time complexity: O(MN) where len(text1) == M and len(text2) == N
        Space complexity: O(MN)
        """
        def lcs(i1: int, i2: int) -> int:
            # We've reached the end of one of the strings (or one of
            # strings is empty), which means that there are no more
            # characters to compare.
            if (i1, i2) in self.cache:
                return self.cache[(i1, i2)]

            if i1 == len(text1) or i2 == len(text2):
                return 0

            if text1[i1] == text2[i2]:
                res = 1 + lcs(i1 + 1, i2 + 1)
            else:
                res = max(lcs(i1 + 1, i2), lcs(i1, i2 + 1))

            self.cache[(i1, i2)] = res
            return res

        self.cache = {}
        return lcs(0, 0)

    def longestCommonSubsequence2(self, text1: str, text2: str) -> int:
        """ Solution using dynamic programming.

        Time complexity: O(MN)
        Space complexity: O(MN)
        """
        matrix = [[0] * (len(text2) + 1) for _ in range(len(text1) + 1)]

        for j in reversed(range(len(text2))):
            for i in reversed(range(len(text1))):
                if text1[i] == text2[j]:
                    matrix[i][j] = 1 + matrix[i+1][j+1]
                else:
                    matrix[i][j] = max(matrix[i+1][j], matrix[i][j+1])

        return matrix[0][0]


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (
                {"text1": "abcde", "text2": "ace"},
                3,
            ),
            (
                {"text1": "nmacebafe", "text2": "ace"},
                3,
            ),
            (
                {"text1": "abc", "text2": "abc"},
                3,
            ),
            (
                {"text1": "abc", "text2": "def"},
                0,
            ),
            (
                {"text1": "ahbzcfg", "text2": "abcdef"},
                4,
            ),
            (
                {"text1": "ahbzcfg", "text2": "fedcba"},
                1,
            ),
            (
                {"text1": "aaaaaa", "text2": "abc"},
                1,
            ),
        ]

    def test_longestCommonSubsequence(self):
        s = Solution()

        for _input, expected in self.input_expected:
            self.assertEqual(expected, s.longestCommonSubsequence(**_input))
            self.assertEqual(expected, s.longestCommonSubsequence2(**_input))


if __name__ == "__main__":
    unittest.main(verbosity=2)
