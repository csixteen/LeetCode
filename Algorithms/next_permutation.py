#!/usr/bin/env python3
# coding: utf-8
import unittest


class Solution:
    def nextPermutation(self, A):
        def reverse(S, start, end):
            while start < end:
                S[start], S[end] = S[end], S[start]
                start += 1
                end -= 1

        k = len(A) - 2
        while k > -1 and A[k] >= A[k+1]:
            k -= 1

        if k == -1:
            reverse(A, 0, len(A) - 1)
            return A

        l = len(A) - 1
        while l > k and A[l] <= A[k]:
            l -= 1

        A[k], A[l] = A[l], A[k]
        reverse(A, k + 1, len(A) - 1)

        return A


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.test_cases = [
            ([1, 2, 3], [1, 3, 2]),
            ([3, 2, 1], [1, 2, 3]),
            ([1, 1, 5], [1, 5, 1])
        ]

    def test_nextPermutation(self):
        s = Solution()
        for i, e in self.test_cases:
            self.assertEqual(s.nextPermutation(i), e)


if __name__ == '__main__':
    unittest.main(verbosity=2)

