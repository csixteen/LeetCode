#!/usr/bin/env python3
# coding: utf-8
import unittest


class Solution:
    def largestRectangleArea(self, A):
        stack = []
        max_area = 0
        for index, height in enumerate(A):
            left = None
            while stack and stack[-1][1] > height:
                left = stack.pop()
                max_area = max(max_area, (index - left[0]) * left[1])

            if left:
                stack.append((left[0], height))
            else:
                stack.append((index, height))

        # This covers the case when we had an increasing sequence
        # of heights, such as test case [1, 2, 3, 4, 5].
        index = len(A)
        while stack:
            left = stack.pop()
            max_area = max(max_area, (index - left[0]) * left[1])

        return max_area


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.test_cases = [
            ([2, 1, 3, 4, 2, 5, 1, 12], 12),
            ([2, 1, 5, 6, 2, 3], 10),
            ([5, 4, 3, 2, 1], 9),
            ([1, 2, 3, 4, 5], 9)
        ]

    def test_largestRectangleArea(self):
        s = Solution()
        for i, e in self.test_cases:
            self.assertEqual(s.largestRectangleArea(i), e)


if __name__ == '__main__':
    unittest.main(verbosity=2)

